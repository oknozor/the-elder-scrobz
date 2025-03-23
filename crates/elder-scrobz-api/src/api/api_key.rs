use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::api_key::{generate_api_key, CreateApiKey};
use elder_scrobz_db::user::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiKey {
    pub api_key: String,
}

#[debug_handler]
pub async fn create_api_key(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> AppResult<Json<ApiKey>> {
    let Some(user) = User::get_by_id(&state.pool, &user_id).await? else {
        return Err(AppError::UserNotFound { id: user_id });
    };

    let key = generate_api_key();
    CreateApiKey {
        sha: key.sha,
        api_key_hash: key.hashed_key,
        user_id: user.id,
    }
    .insert(&state.pool)
    .await?;

    Ok(Json(ApiKey { api_key: key.key }))
}
