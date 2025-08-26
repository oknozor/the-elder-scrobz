use autometrics::prometheus_exporter;
use axum::Extension;
use axum::routing::get;
use elder_scrobz_api::api::{ApiDoc, router};
use elder_scrobz_api::oauth::client::get_oauth2_client;
use elder_scrobz_crawler::{DiscogsConfig, MetadataClient, NavidromeConfig, ScrobbleCrawler};
use elder_scrobz_db::build_pg_pool;
use elder_scrobz_settings::Settings;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal::unix::{SignalKind, signal};
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

    let app = router(settings.debug, pool.clone())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(MetadataClient::new(
            settings.discogs_key.clone(),
            settings.discogs_secret.clone(),
            settings.navidrome_username.clone(),
            settings.navidrome_password.clone(),
            settings.navidrome_server_url.clone(),
        )))
        .layer(Extension(oauth_client))
        .layer(Extension(settings.clone()))
        .with_state(pool.clone());

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
        .nest_service("/coverarts", ServeDir::new(&coverart_path))
        .fallback_service(serve_frontend);

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
        coverart_path,
        DiscogsConfig {
            key: settings.discogs_key.clone(),
            secret: settings.discogs_secret.clone(),
        },
        NavidromeConfig {
            username: settings.navidrome_username.clone(),
            password: settings.navidrome_password.clone(),
            server_url: settings.navidrome_server_url.clone(),
        },
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
