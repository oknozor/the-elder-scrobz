use crate::api::app;
use crate::settings::Settings;
use elder_scrobz_db::{PgPool, PgPoolOptions};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod error;
mod settings;

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
        let pool = build_pg_pool(&settings).await;
        let settings = Arc::new(settings);
        Ok(AppState { pool, settings })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "tower_http=debug,axum=debug,elder_scrobz_api=debug,elder_scrobz_api=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::init().await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], state.settings.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("listening on {}", listener.local_addr().unwrap());

    let app = app().layer(TraceLayer::new_for_http()).with_state(state);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn build_pg_pool(settings: &Settings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&settings.database_url())
        .await
        .expect("can't connect to database")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app;
    use crate::test_helper::scrobble_fixture;
    use axum::{http::Request, http::StatusCode};
    use elder_scrobz_db::user::CreateUser;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_user_ok() -> anyhow::Result<()> {
        let state = AppState::init().await?;
        let app = app().with_state(state);

        let body = serde_json::to_string(&CreateUser {
            username: "oknozor".to_string(),
            email: "paul.delafosse@protonmail.com".to_string(),
        })?;

        let request = Request::builder()
            .method("POST")
            .uri("/users")
            .header("Content-Type", "application/json")
            .body(body)?;

        let response = ServiceExt::oneshot(app, request).await?;

        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn create_api_key_ok() -> anyhow::Result<()> {
        let state = AppState::init().await?;
        let app = app().with_state(state);

        let request = Request::builder()
            .method("POST")
            .uri("/users/224d01d6-8a79-4ad9-bfc9-1bb89182fbf1/api-key/create")
            .header("Content-Type", "application/json")
            .body(axum::body::Body::empty())?;

        let response = ServiceExt::oneshot(app, request).await?;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let body = String::from_utf8(bytes.to_vec())?;
        println!("{}", body);

        Ok(())
    }

    #[tokio::test]
    async fn submit_listens_ok() -> anyhow::Result<()> {
        let scrobble = scrobble_fixture()?;

        let request = Request::builder()
            .method("POST")
            .uri("/1/submit-listens")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Token HUGFemCse5VgrfxVqjvcRRFoMfFuiHbXXUe972Nycnw=",
            )
            .body(scrobble)?;

        let state = AppState::init().await?;
        let app = app().with_state(state);
        let response = ServiceExt::oneshot(app, request).await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let body = String::from_utf8(bytes.to_vec())?;

        assert_eq!(body, "Something went wrong: it failed!");
        Ok(())
    }

    #[tokio::test]
    async fn submit_listens_no_token() -> anyhow::Result<()> {
        let scrobble = scrobble_fixture()?;

        let request = Request::builder()
            .method("POST")
            .uri("/1/submit-listens")
            .header("Content-Type", "application/json")
            .body(scrobble)?;

        let state = AppState::init().await?;
        let app = app().with_state(state);

        let response = app.oneshot(request).await?;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}
