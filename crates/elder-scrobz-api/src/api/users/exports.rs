use crate::error::{AppError, AppResult};
use crate::oauth::AuthenticatedUser;
use autometrics::autometrics;
use axum::body::Body;
use axum::response::Response;
use axum::Extension;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;
use elder_scrobz_db::PgPool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[debug_handler]
#[utoipa::path(get, path = "/export", summary = "Export listens")]
#[autometrics]
pub async fn export_listens(
    user: AuthenticatedUser,
    Extension(db): Extension<PgPool>,
) -> AppResult<axum::response::Response> {
    let mut all_json_lines = String::new();
    let mut offset = 0;
    let limit = 100;

    while let Ok(scrobbles) = RawScrobble::by_user_id(&db, &user.name, (limit, offset)).await {
        if scrobbles.is_empty() {
            break;
        }

        for scrobble in scrobbles {
            let json_line = match serde_json::to_string(&scrobble) {
                Ok(mut line) => {
                    line.push('\n');
                    line
                }
                Err(_) => continue,
            };

            all_json_lines.push_str(&json_line);
        }

        offset += limit;
    }

    let body = Body::from(all_json_lines);

    let response = Response::builder()
        .header("content-type", "application/x-ndjson")
        .header(
            "content-disposition",
            "attachment; filename=\"listens.jsonl\"",
        )
        .body(body)
        .map_err(|err| AppError::Internal(err.to_string()))?;

    Ok(response)
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(export_listens))
}
