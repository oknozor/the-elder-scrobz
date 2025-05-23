use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use utoipa::ToSchema;

pub type AppResult<T> = Result<T, AppError>;

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into().to_string())
    }
}

#[derive(Debug, ToSchema)]
pub enum AppError {
    #[schema(example = "Internal error")]
    Internal(String),
    #[schema(example = "User not found")]
    UserNotFound { id: String },
    #[schema(example = "Scrobble not found")]
    ScrobbleNotFound { id: String },
    #[schema(example = "Track not found")]
    TrackNotFound { id: String },
    #[schema(example = "Album not found")]
    AlbumNotFound { id: String },
    #[schema(example = "Operation not allowed")]
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
            AppError::ScrobbleNotFound { id } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("Scrobble with id {id} not found")
                })),
            ),
            AppError::TrackNotFound { id } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("Track with id {id} not found")
                })),
            ),
            AppError::AlbumNotFound { id } => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("Album  with id {id} not found")
                })),
            ),
        }
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::admin::router;
    use crate::test_helper::scrobble_fixture;
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

        let (app, _) = router().split_for_parts();

        let response = app.oneshot(request).await?;

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let html = String::from_utf8(bytes.to_vec())?;

        assert_eq!(html, "Something went wrong: it failed!");
        Ok(())
    }
}
