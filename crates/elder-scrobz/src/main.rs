use autometrics::prometheus_exporter;
use axum::Extension;
use axum::routing::get;
use elder_scrobz_api::api::{ApiDoc, router};
use elder_scrobz_api::oauth::client::get_oauth2_client;
use elder_scrobz_api::settings::Settings;
use elder_scrobz_crawler::{MetadataClient, ScrobbleResolver};
use elder_scrobz_db::build_pg_pool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
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

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=debug,scrobz=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = Settings::get()?;
    let settings = Arc::new(settings);
    let pool = build_pg_pool(&settings.database_url).await;
    let coverart_path = settings.coverart_path.clone();

    elder_scrobz_db::migrate_db(&pool).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("listening on {}", listener.local_addr()?);
    let oauth_client = get_oauth2_client(&settings).await?;

    let app = router(settings.debug)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool.clone()))
        .layer(Extension(MetadataClient::new(
            settings.discogs_token.clone(),
        )))
        .layer(Extension(oauth_client))
        .layer(Extension(settings.clone()));

    #[cfg(debug_assertions)]
    let app = {
        info!("Enabling CORS for local development. Don't do this in production.");
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
        .nest_service("/coverarts", ServeDir::new(&coverart_path))
        .fallback_service(serve_frontend);

    // TODO: cancel condition
    let token = CancellationToken::new();
    let mut resolver = ScrobbleResolver::create(
        pool.clone(),
        coverart_path,
        settings.discogs_token.clone(),
        token.child_token(),
    )
    .await?;
    let (a, b) = tokio::join!(axum::serve(listener, router), resolver.run());
    a?;
    b?;
    Ok(())
}
