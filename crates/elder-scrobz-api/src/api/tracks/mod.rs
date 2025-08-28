use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::tracks::TrackWithPlayCount as TrackWithPlayCountEntity;
use elder_scrobz_db::PgPool;
use elder_scrobz_model::track::{Track, TrackWithPlayCount};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/{id}",
    summary = "Track by id",
    responses(
        (status = 200, description = "A track", body = Track, content_type = "application/json"),
        (status = 404, description = "Track not found", body = AppError)
    ),
    tag = crate::api::TRACKS_TAG
)]
#[autometrics]
pub async fn by_id(
    Path(id): Path<String>,
    State(db): State<PgPool>,
) -> AppResult<Json<TrackWithPlayCount>> {
    match TrackWithPlayCountEntity::by_id(&id, &db).await? {
        Some(track) => Ok(Json(TrackWithPlayCount::from(track))),
        None => Err(AppError::TrackNotFound { id: id.to_string() }),
    }
}

pub(crate) fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new().routes(routes!(by_id))
}
