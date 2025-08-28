use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::releases::AlbumDetails as AlbumDetailsEntity;
use elder_scrobz_db::PgPool;
use elder_scrobz_model::album::AlbumDetails;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/{id}",
    summary = "Album by id",
    responses(
        (status = 200, description = "An album", body = AlbumDetails, content_type = "application/json"),
        (status = 404, description = "Album not found", body = AppError)
    ),
    tag = crate::api::ALBUMS_TAG
)]
#[autometrics]
pub async fn by_id(
    Path(id): Path<String>,
    State(db): State<PgPool>,
) -> AppResult<Json<AlbumDetails>> {
    let album = AlbumDetailsEntity::by_id(&id, &db).await?;
    Ok(Json(AlbumDetails::from(album)))
}

pub(crate) fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new().routes(routes!(by_id))
}
