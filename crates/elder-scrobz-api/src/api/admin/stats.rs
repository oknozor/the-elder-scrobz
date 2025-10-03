use crate::error::AppResult;
use crate::state::AppState;
use autometrics::autometrics;
use axum::extract::State;
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::stats::Stats;

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
pub async fn stats(State(state): State<AppState>) -> AppResult<Json<Stats>> {
    Ok(Json(Stats::get(&state.db).await?))
}
