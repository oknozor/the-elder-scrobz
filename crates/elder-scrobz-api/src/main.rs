use crate::api::router;
use crate::oauth::verify_bearer_token;
use crate::settings::Settings;
use axum::middleware;
use elder_scrobz_db::build_pg_pool;
use elder_scrobz_db::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod error;
mod settings;

mod oauth;
#[cfg(test)]
mod test_helper;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    settings: Arc<Settings>,
}

impl AppState {
    async fn init() -> anyhow::Result<Self> {
        let settings = Settings::get()?;
        let pool = build_pg_pool(&settings.database_url).await;
        let settings = Arc::new(settings);
        Ok(AppState { pool, settings })
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
            (name = "scrobz", description = "The Elder Scrobz")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tower_http=debug,scrobz=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::init().await?;
    elder_scrobz_db::migrate_db(&state.pool).await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], state.settings.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("listening on {}", listener.local_addr()?);
    let app = router()
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let (mut router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", app)
        .split_for_parts();

    if !state.settings.debug {
        router = router.layer(middleware::from_fn_with_state(state, verify_bearer_token))
    }

    let router =
        router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    axum::serve(listener, router).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::api::user::UserCreated;
    use crate::router;
    use crate::test_helper::{scrobble_fixture, start_postgres};
    use axum::response::Response;
    use axum::{http::Request, http::StatusCode};
    use elder_scrobz_db::user::CreateUser;
    use http_body_util::BodyExt;
    use speculoos::prelude::*;
    use tower::ServiceExt;

    #[tokio::test]
    async fn submit_listens() -> anyhow::Result<()> {
        let (state, _container) = start_postgres().await?;
        let (app, _) = router().with_state(state).split_for_parts();

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
        let user: UserCreated = serde_json::from_str(&body)?;

        let request = Request::builder()
            .method("POST")
            .uri(format!("/users/{}/api-key/create", user.user_id))
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
