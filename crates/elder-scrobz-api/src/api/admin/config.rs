use crate::{error::AppResult, state::AppState};
use autometrics::autometrics;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::configs::GlobalConfig;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/config",
    summary = "Get user config",
    responses(
        (status = 200, description = "User config", body = GlobalConfig, content_type = "application/json"),
        (status = 401, description = "Not authenticated"),
    ),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn get_config(State(state): State<AppState>) -> AppResult<Json<GlobalConfig>> {
    let user_config = GlobalConfig::get(&state.db).await?;
    Ok(Json(user_config))
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/config",
    summary = "Set user config",
    responses(
        (status = 200, description = "User config updated", body = GlobalConfig, content_type = "application/json"),
        (status = 401, description = "Not authenticated"),
    ),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn set_config(
    State(state): State<AppState>,
    Json(config): Json<GlobalConfig>,
) -> AppResult<Json<GlobalConfig>> {
    config.save(&state.db).await?;
    Ok(Json(config))
}
