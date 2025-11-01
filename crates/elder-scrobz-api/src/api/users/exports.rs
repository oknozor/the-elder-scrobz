use crate::api::user_from_session;
use crate::error::{AppError, AppResult};
use crate::oauth::router::Session;
use crate::state::AppState;
use autometrics::autometrics;
use axum::body::Body;
use axum::extract::State;
use axum::response::Response;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[debug_handler]
#[utoipa::path(get, path = "/export", summary = "Export listens")]
#[autometrics]
pub async fn export_listens(
    session: Session,
    State(state): State<AppState>,
) -> AppResult<axum::response::Response> {
    let user = user_from_session(session)?;
    let mut all_json_lines = String::new();
    let mut offset = 0;
    let limit = 100;

    while let Ok(scrobbles) =
        RawScrobble::by_user_id(&state.db, &user.username, (limit, offset)).await
    {
        if scrobbles.is_empty() {
            break;
        }

        for scrobble in scrobbles {
            let json_line = match serde_json::to_string(&scrobble.data) {
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

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(export_listens))
}
