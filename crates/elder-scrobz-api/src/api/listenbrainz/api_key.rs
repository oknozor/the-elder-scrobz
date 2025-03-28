use crate::api::listenbrainz::Token;
use crate::error::{AppError, AppResult};
use crate::oauth::AuthenticatedUser;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
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
    path = "/users/api-key/create",
    responses(
        (status = 200, description = "Create a new user ApiKey", body = ApiKeyCreated),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::API_KEYS_TAG
)]
pub async fn create_api_key(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> AppResult<Json<ApiKeyCreated>> {
    let Some(user) = User::get_by_username(&state.pool, &user.name).await? else {
        return Err(AppError::UserNotFound { id: user.name });
    };

    let key = generate_api_key();
    CreateApiKey {
        sha: key.sha,
        api_key_hash: key.hashed_key,
        username: user.username,
    }
    .insert(&state.pool)
    .await?;

    Ok(Json(ApiKeyCreated { api_key: key.key }))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TokenValidation {
    pub valid: bool,
    pub code: i64,
    pub user_name: Option<String>,
    pub message: String,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/validate-token",
    params(
        ("Authorization" = String, Header, description = "Token to validate. Format: `Token <token>`")
    ),
    responses(
        (status = 200, description = "The user token is valid/invalid.", content_type = "application/json", body = TokenValidation)
    ),
    tag = crate::api::API_KEYS_TAG
)]
pub async fn validate_token(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Token>>,
) -> AppResult<Json<TokenValidation>> {
    let Some(token) = auth.0.token()? else {
        return Err(AppError::Unauthorized("Missing token".to_string()));
    };

    Ok(
        match User::get_user_id_by_api_key(&state.pool, token).await? {
            None => Json(TokenValidation {
                valid: false,
                code: 1,
                user_name: None,
                message: "invalid token".to_string(),
            }),
            Some(user) => Json(TokenValidation {
                valid: true,
                code: 0,
                user_name: Some(user.username),
                message: "token is valid".to_string(),
            }),
        },
    )
}
