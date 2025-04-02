use crate::api::listenbrainz::Token;
use crate::error::{AppError, AppResult};
use crate::oauth::AuthenticatedUser;
use autometrics::autometrics;
use axum::{Extension, Json};
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use elder_scrobz_db::api_key::{generate_api_key, CreateApiKey};
use elder_scrobz_db::user::{ApiKey, User};
use elder_scrobz_db::PgPool;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiKeyCreated {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateApiKeyRequest {
    pub label: String,
}

#[debug_handler]
#[utoipa::path(
    post,
    summary = "Create api key",
    path = "/users/api-keys",
    responses(
        (status = 200, description = "Create a new user ApiKey", body = ApiKeyCreated),
        (status = 404, description = "User not found", body = AppError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = crate::api::API_KEYS_TAG
)]
#[autometrics]
pub async fn create_api_key(
    user: AuthenticatedUser,
    Extension(db): Extension<PgPool>,
    Json(payload): Json<CreateApiKeyRequest>
) -> AppResult<Json<ApiKeyCreated>> {
    let Some(user) = User::get_by_username(&db, &user.name).await? else {
        return Err(AppError::UserNotFound { id: user.name });
    };

    let key = generate_api_key();
    CreateApiKey {
        sha: key.sha,
        api_key_hash: key.hashed_key,
        username: user.username,
        label: payload.label,
    }
    .insert(&db)
    .await?;

    Ok(Json(ApiKeyCreated { api_key: key.key }))
}

#[debug_handler]
#[utoipa::path(
    get,
    summary = "List api keys",
    path = "/users/api-keys",
    responses(
        (status = 200, description = "Create a new user ApiKey", body = Vec<ApiKey>),
        (status = 404, description = "User not found", body = AppError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = crate::api::API_KEYS_TAG
)]
#[autometrics]
pub async fn get_api_keys(
    user: AuthenticatedUser,
    Extension(db): Extension<PgPool>,
) -> Result<Json<Vec<ApiKey>>, AppError> {
    let Some(user) = User::get_by_username(&db, &user.name).await? else {
        return Err(AppError::UserNotFound { id: user.name });
    };

    Ok(Json(user.get_api_keys(&db).await ?))
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
    summary = "Validate token",
    params(
        ("Authorization" = String, Header, description = "Token to validate. Format: `Token <token>`")
    ),
    responses(
        (status = 200, description = "The user token is valid/invalid.", content_type = "application/json", body = TokenValidation)
    ),
    tag = crate::api::API_KEYS_TAG
)]
pub async fn validate_token(
    Extension(db): Extension<PgPool>,
    TypedHeader(auth): TypedHeader<Authorization<Token>>,
) -> AppResult<Json<TokenValidation>> {
    let Some(token) = auth.0.token()? else {
        return Err(AppError::Unauthorized("Missing token".to_string()));
    };

    Ok(match User::get_user_id_by_api_key(&db, token).await? {
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
    })
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeleteApiKeyResponse {
    pub success: bool,
}

#[debug_handler]
#[utoipa::path(
    delete,
    summary = "Delete api key",
    path = "/users/api-keys/{id}",
    params(
        ("id" = i32, Path, description = "API key ID to delete")
    ),
    responses(
        (status = 200, description = "API key deleted successfully", body = DeleteApiKeyResponse),
        (status = 404, description = "User not found or API key not found", body = AppError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = crate::api::API_KEYS_TAG
)]
#[autometrics]
pub async fn delete_api_key(
    user: AuthenticatedUser,
    Extension(db): Extension<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> AppResult<Json<DeleteApiKeyResponse>> {
    let Some(user) = User::get_by_username(&db, &user.name).await? else {
        return Err(AppError::UserNotFound { id: user.name });
    };

    let success = user.delete_api_key(&db, id).await?;

    if !success {
        return Err(AppError::Internal(format!("API key with id {} not found", id)));
    }

    Ok(Json(DeleteApiKeyResponse { success }))
}
