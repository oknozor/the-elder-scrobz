use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::artists::Artist as ArtistEntity;
use elder_scrobz_db::PgPool;
use elder_scrobz_model::artist::Artist;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/{id}",
    summary = "Artist by id",
    responses(
        (status = 200, description = "An artist", body = Artist, content_type = "application/json"),
        (status = 404, description = "Artist not found", body = AppError)
    ),
    tag = crate::api::ARTISTS_TAG
)]
#[autometrics]
pub async fn by_id(Path(id): Path<String>, State(db): State<PgPool>) -> AppResult<Json<Artist>> {
    let artist = ArtistEntity::by_id(&id, &db).await?;
    Ok(Json(artist.into()))
}

pub(crate) fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new().routes(routes!(by_id))
}
