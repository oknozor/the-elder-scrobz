use crate::error::AppResult;
use autometrics::autometrics;
use axum::extract::Path;
use axum::Extension;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::Release;
use elder_scrobz_db::PgPool;

#[debug_handler]
#[utoipa::path(
    patch,
    path = "/releases/{id}/remove-coverart",
    summary = "Remove a release coverart",
    description = "Remove a release coverart, this can be useful when coverart archive return invalid images",
    responses((status = 200)),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn remove_coverart(
    Path(id): Path<String>,
    Extension(db): Extension<PgPool>,
) -> AppResult<()> {
    Release::remove_coverart(&id, &db).await?;
    Ok(())
}
