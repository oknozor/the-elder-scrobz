use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/scrobbles/{id}",
    responses(
        (status = 200, description = "Get a raw scrobble by id", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<RawScrobble>> {
    match RawScrobble::get_by_id(&state.pool, &id).await? {
        None => Err(AppError::ScrobbleNotFound { id }),
        Some(scrobble) => Ok(Json(scrobble)),
    }
}
