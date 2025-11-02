use autometrics::prometheus_exporter;
use axum::Extension;
use axum::body::Body;
use axum::extract::Request;
use axum::routing::get;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::SessionPgPool;
use elder_scrobz_api::api::{ApiDoc, router};
use elder_scrobz_api::oauth;
use elder_scrobz_api::oauth::client::OauthClient;
use elder_scrobz_api::oauth::user::AuthenticatedUser;
use elder_scrobz_api::state::AppState;
use elder_scrobz_crawler::{MetadataClient, ScrobbleCrawler};
use elder_scrobz_db::{PgPool, build_pg_pool};
use elder_scrobz_model::events::ScrobzEvent;
use elder_scrobz_settings::Settings;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{Span, info, info_span, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    prometheus_exporter::init();

    #[cfg(debug_assertions)]
    {
        dotenv::dotenv().ok();
        dotenv::from_filename(".secret.env").ok();
    }

    let settings = Arc::new(Settings::get()?);
    let pool = build_pg_pool(&settings.database_url).await;

    elder_scrobz_db::migrate_db(&pool).await?;

    let session_config = SessionConfig::default().with_table_name("sessions_table");
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config).await?;

    let auth_config = AuthConfig::<String>::default();
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("listening on {}", listener.local_addr()?);
    let oauth_client = OauthClient::build(&settings.oidc).await?;
    let (sse_sender, _sse_receiver) = broadcast::channel::<ScrobzEvent>(100);

    let state = AppState::new(pool.clone(), sse_sender.clone());

    let app = router(settings.debug);

    #[cfg(debug_assertions)]
    let app = {
        warn!("Enabling CORS for local development. Don't do this in production.");
        use axum::http::HeaderValue;
        use tower_http::cors::{Any, CorsLayer};
        app.layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(Any)
                .allow_methods(Any),
        )
    };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", app)
        .split_for_parts();

    let router = router.route(
        "/metrics",
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    if settings.debug {
        warn!("Debug mode enabled, running without oauth2 security");
    }

    let serve_frontend = ServeDir::new("/app/frontend")
        .not_found_service(ServeFile::new("/app/frontend/index.html"));

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .nest("/auth", oauth::router::router())
        .nest_service("/coverarts", ServeDir::new(&settings.coverart_path))
        .fallback_service(serve_frontend)
        .layer(Extension(MetadataClient::new(
            settings.discogs.key.clone(),
            settings.discogs.secret.clone(),
            settings.navidrome.username.clone(),
            settings.navidrome.password.clone(),
            settings.navidrome.server_url.clone(),
        )))
        .layer(Extension(settings.clone()))
        .layer(
            AuthSessionLayer::<AuthenticatedUser, String, SessionPgPool, PgPool>::new(Some(
                pool.clone(),
            ))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store.clone()))
        .layer(Extension(oauth_client))
        .layer(build_trace_layer())
        .with_state(state);

    let token = CancellationToken::new();
    let token_clone = token.clone();

    tokio::spawn(async move {
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");
        sigterm.recv().await; // Wait for SIGTERM
        info!("Received SIGTERM, shutting down...");
        token_clone.cancel(); // Notify main task to exit
    });

    let mut crawler = ScrobbleCrawler::create(
        pool.clone(),
        settings.coverart_path.clone(),
        settings.discogs.clone(),
        settings.navidrome.clone(),
        token.clone(),
    )
    .await?;

    let axum_token = token.clone();
    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        axum_token.cancelled().await;
    });
    let crawler_task = crawler.run();

    tokio::select! {
        result = server => {
            info!("Server shutdown: {:?}", result);
            result?;
        },
        result = crawler_task => {
            info!("Crawler shutdown: {:?}", result);
            result?;
        },
        _ = token.cancelled() => {
            info!("Shutdown signal received");
        }
    }

    Ok(())
}

fn build_trace_layer()
-> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, impl Fn(&Request<Body>) -> Span + Clone> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "tower_http=debug,elder_scrobz_api=debug,scrobz=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let safe_path = mask_token_in_uri(request.uri());
        info_span!("http-request", method = %request.method(), path = %safe_path)
    })
}

fn mask_token_in_uri(uri: &axum::http::Uri) -> String {
    let path = uri.path();
    if let Some(query) = uri.query() {
        // Replace token=... with token=<user-token>
        let safe_query = query
            .split('&')
            .map(|param| {
                if param.starts_with("token=") {
                    "token=<user-token>"
                } else {
                    param
                }
            })
            .collect::<Vec<_>>()
            .join("&");
        return format!("{}?{}", path, safe_query);
    }
    path.to_string()
}
