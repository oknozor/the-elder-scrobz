use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;
use elder_scrobz_db::PgPool;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/scrobbles/{id}",
    summary = "Get scrobble by id",
    responses(
        (status = 200, body = RawScrobble, description = "Get a raw scrobble by id", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn get_by_id(
    Path(id): Path<String>,
    State(db): State<PgPool>,
) -> AppResult<Json<RawScrobble>> {
    match RawScrobble::get_by_id(&db, &id).await? {
        None => Err(AppError::ScrobbleNotFound { id }),
        Some(scrobble) => Ok(Json(scrobble)),
    }
}
