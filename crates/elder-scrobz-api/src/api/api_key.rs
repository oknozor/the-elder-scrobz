use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::api_key::{generate_api_key, CreateApiKey};
use elder_scrobz_db::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiKeyCreated {
    pub api_key: String,
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/users/{id}/api-key/create",
    responses(
        (status = 200, description = "Create a new user ApiKey", body = ApiKeyCreated),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn create_api_key(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> AppResult<Json<ApiKeyCreated>> {
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

    Ok(Json(ApiKeyCreated { api_key: key.key }))
}
