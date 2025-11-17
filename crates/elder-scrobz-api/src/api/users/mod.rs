use crate::api::pagination::{PageQuery, PaginatedResponse, ToOffset};
use crate::error::{AppError, AppResult};
use crate::oauth::user::AuthenticatedUser;
use crate::state::AppState;
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::user::{CreateUser, User as DbUser, User, UserWithRole};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod config;
pub mod exports;
pub mod imports;

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentUserInfo {
    pub username: String,
    pub admin: bool,
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(get_users))
        .routes(routes!(get_current_user))
        .routes(routes!(config::get_user_config))
        .routes(routes!(config::set_user_config))
        .merge(imports::router())
        .merge(exports::router())
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/",
    summary = "Create user",
    responses(
        (status = 200, description = "User Created", body = User, content_type = "application/json"),
    ),
    tag = crate::api::USERS_TAG
)]
#[autometrics]
pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> AppResult<Json<UserWithRole>> {
    Ok(Json(user.insert(&state.db).await?))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/",
    summary = "Get user",
    params(PageQuery),
    responses(
        (status = 200, description = "All users", body = PaginatedResponse<User>, content_type = "application/json"),
    ),
    tag = crate::api::USERS_TAG
)]
#[autometrics]
pub async fn get_users(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<PaginatedResponse<User>>> {
    let (total, users) = DbUser::all(&state.db, query.per_page(), query.to_offset()).await?;
    let response = PaginatedResponse::from_query(users, total, query);

    Ok(Json(response))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/me",
    summary = "Get current user",
    responses(
        (status = 200, description = "Current user info", body = CurrentUserInfo, content_type = "application/json"),
        (status = 401, description = "Not authenticated"),
    ),
    tag = crate::api::USERS_TAG
)]
#[autometrics]
pub async fn get_current_user(
    authenticated_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> AppResult<Json<CurrentUserInfo>> {
    let user = DbUser::get_by_username_with_permission(&state.db, &authenticated_user.username)
        .await?
        .ok_or(AppError::UserNotFound {
            id: authenticated_user.username,
        })?;

    Ok(Json(CurrentUserInfo {
        username: user.username,
        admin: user.admin,
    }))
}
