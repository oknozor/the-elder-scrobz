use crate::error::AppResult;
use autometrics::autometrics;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::stats::Stats;
use elder_scrobz_db::PgPool;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/stats",
    summary = "Database statistics",
    description = "Statistics on database metadata",
    responses(
        (status = 200, body = Stats, description = "Return detailed stats about the scrobble database", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn stats(Extension(db): Extension<PgPool>) -> AppResult<Json<Stats>> {
    Ok(Json(Stats::get(&db).await?))
}
