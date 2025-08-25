use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::listens::artists::Artist;
use elder_scrobz_db::{PgPool, WithLocalImage};
use elder_scrobz_settings::Settings;
use std::sync::Arc;
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
pub async fn by_id(
    Path(id): Path<String>,
    State(db): State<PgPool>,
    Extension(settings): Extension<Arc<Settings>>,
) -> AppResult<Json<Artist>> {
    let artist = Artist::by_id(&id, &db)
        .await?
        .with_local_image(&settings.coverart_path);
    Ok(Json(artist))
}

pub(crate) fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new().routes(routes!(by_id))
}
