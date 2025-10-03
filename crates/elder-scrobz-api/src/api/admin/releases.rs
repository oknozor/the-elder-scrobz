use crate::error::AppResult;
use crate::state::AppState;
use autometrics::autometrics;
use axum::extract::{Path, State};
use axum_macros::debug_handler;
use elder_scrobz_db::listens::releases::Release;

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
    State(state): State<AppState>,
) -> AppResult<()> {
    Release::remove_coverart(&id, &state.db).await?;
    Ok(())
}
