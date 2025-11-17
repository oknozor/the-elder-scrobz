use autometrics::autometrics;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::configs::UserConfig;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    error::{AppError, AppResult},
    oauth::user::AuthenticatedUser,
    state::AppState,
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_user_config, set_user_config))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/config",
    summary = "Get user config",
    responses(
        (status = 200, description = "User config", body = UserConfig, content_type = "application/json"),
        (status = 401, description = "Not authenticated"),
    ),
    tag = crate::api::USERS_TAG
)]
#[autometrics]
pub async fn get_user_config(
    authenticated_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> AppResult<Json<UserConfig>> {
    let user_config = UserConfig::get(&state.db, &authenticated_user.username).await?;
    Ok(Json(user_config))
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/config",
    summary = "Set user config",
    responses(
        (status = 200, description = "User config updated", body = UserConfig, content_type = "application/json"),
        (status = 401, description = "Not authenticated"),
    ),
    tag = crate::api::USERS_TAG
)]
#[autometrics]
pub async fn set_user_config(
    authenticated_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(user_config): Json<UserConfig>,
) -> AppResult<Json<UserConfig>> {
    if authenticated_user.username != user_config.username {
        return Err(AppError::Unauthorized(format!(
            "User {} is not authorized to update user {}",
            authenticated_user.username, user_config.username
        )));
    }

    user_config.save(&state.db).await?;
    Ok(Json(user_config))
}
