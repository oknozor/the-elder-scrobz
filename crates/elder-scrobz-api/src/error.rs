use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}

pub enum AppError {
    Internal(anyhow::Error),
    UserNotFound { id: String },
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": err.to_string()
                })),
            ),
            AppError::UserNotFound { id } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": format!("User with id {id} not found")
                })),
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": msg
                })),
            ),
        }
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::scrobble_fixture;
    use crate::{app, build_pg_pool, AppState};
    use axum::{http::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_main_page() -> anyhow::Result<()> {
        let scrobble = scrobble_fixture()?;

        let request = Request::builder()
            .method("POST")
            .uri("/1/submit-listens")
            .header("Content-Type", "application/json")
            .body(scrobble)?;

        let state = AppState {
            pool: build_pg_pool().await,
        };
        let app = app().with_state(state);

        let response = app.oneshot(request).await?;

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let html = String::from_utf8(bytes.to_vec())?;

        assert_eq!(html, "Something went wrong: it failed!");
        Ok(())
    }
}
