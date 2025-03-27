use crate::api::{router, ApiDoc};
use elder_scrobz_resolver::ScrobbleResolver;
use state::AppState;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod error;
mod oauth;
mod settings;
mod state;

#[cfg(test)]
mod test_helper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let state = AppState::init().await?;
    let coverart_path = state.settings.coverart_path.clone();
    let pool = state.pool.clone();
    elder_scrobz_db::migrate_db(&state.pool).await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], state.settings.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("listening on {}", listener.local_addr()?);
    let app = router(state.clone())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

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

    if state.settings.debug {
        warn!("Debug mode enabled, running without oauth2 security");
    }

    let serve_frontend = ServeDir::new("/app/frontend")
        .not_found_service(ServeFile::new("/app/frontend/index.html"));

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .nest_service("/coverarts", ServeDir::new(&coverart_path))
        .fallback_service(serve_frontend);

    let mut resolver = ScrobbleResolver::create(pool.clone(), coverart_path).await?;

    let (a, b) = tokio::join!(axum::serve(listener, router), resolver.listen());
    a?;
    b?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::router;
    use crate::test_helper::{scrobble_fixture, start_postgres};
    use axum::response::Response;
    use axum::{http::Request, http::StatusCode};
    use elder_scrobz_db::user::{CreateUser, User};
    use http_body_util::BodyExt;
    use speculoos::prelude::*;
    use tower::ServiceExt;

    #[tokio::test]
    async fn submit_listens() -> anyhow::Result<()> {
        let (state, _container) = start_postgres().await?;
        let (app, _) = router(state.clone()).with_state(state).split_for_parts();

        let body = serde_json::to_string(&CreateUser {
            username: "oknozor".to_string(),
            email: "paul.delafosse@protonmail.com".to_string(),
        })?;

        let request = Request::builder()
            .method("POST")
            .uri("/users")
            .header("Content-Type", "application/json")
            .body(body)?;

        let response = ServiceExt::oneshot(app.clone(), request).await?;
        assert_that!(response.status()).is_equal_to(StatusCode::OK);

        let body = body_to_string(response).await?;
        let user: User = serde_json::from_str(&body)?;

        let request = Request::builder()
            .method("POST")
            .uri(format!("/users/{}/api-key/create", user.username))
            .header("Content-Type", "application/json")
            .body(axum::body::Body::empty())?;

        let response = ServiceExt::oneshot(app.clone(), request).await?;
        assert_that!(response.status()).is_equal_to(StatusCode::OK);

        let body = body_to_string(response).await?;
        #[derive(serde::Deserialize)]
        struct ApiKeyCreated {
            api_key: String,
        }

        let api_key: ApiKeyCreated = serde_json::from_str(&body)?;

        let scrobble = scrobble_fixture()?;

        let request = Request::builder()
            .method("POST")
            .uri("/1/submit-listens")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Token {}", api_key.api_key))
            .body(scrobble)?;

        let response = ServiceExt::oneshot(app.clone(), request).await?;
        assert_that!(response.status()).is_equal_to(StatusCode::OK);

        Ok(())
    }

    async fn body_to_string(response: Response) -> anyhow::Result<String> {
        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let body = String::from_utf8(bytes.to_vec())?;
        Ok(body)
    }
}
