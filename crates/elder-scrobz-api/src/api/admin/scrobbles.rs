use crate::api::pagination::{PageQuery, PaginatedResponse, ToOffset};
use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::dlc::ErroredScrobble;
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;
use elder_scrobz_db::PgPool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

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

#[debug_handler]
#[utoipa::path(
    get,
    path = "/scrobbles/dlc",
    summary = "Get all errroed scrobble",
    responses(
        (status = 200, body = Vec<ErroredScrobble>, description = "Get all errroed scrobble", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn get_all_errored(
    Query(page): Query<PageQuery>,
    State(db): State<PgPool>,
) -> AppResult<Json<PaginatedResponse<ErroredScrobble>>> {
    let (scrobbles, total) = ErroredScrobble::all(&db, page.per_page(), page.to_offset()).await?;
    let pagination = PaginatedResponse::from_query(scrobbles, total, page);
    Ok(Json(pagination))
}

pub(crate) fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(get_by_id))
        .routes(routes!(get_all_errored))
}
