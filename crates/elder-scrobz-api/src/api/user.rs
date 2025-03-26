use crate::api::PageQuery;
use crate::error::AppResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::user::{CreateUser, User as DbUser, User};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(get_users))
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = 200, description = "User Created", body = User, content_type = "application/json"),
    ),
    tag = crate::api::USERS_TAG
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> AppResult<Json<User>> {
    Ok(Json(user.insert(&state.pool).await?))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "All users", body = Vec<User>, content_type = "application/json"),
    ),
    tag = crate::api::USERS_TAG
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<Vec<User>>> {
    Ok(Json(
        DbUser::all(&state.pool, query.page_size, query.page - 1).await?,
    ))
}
