use crate::api::PageQuery;
use crate::error::AppResult;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::user::{CreateUser, User as DbUser, User};
use elder_scrobz_db::PgPool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(get_users))
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
pub async fn create_user(
    Extension(db): Extension<PgPool>,
    Json(user): Json<CreateUser>,
) -> AppResult<Json<User>> {
    Ok(Json(user.insert(&db).await?))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/",
    summary = "Get user",
    responses(
        (status = 200, description = "All users", body = Vec<User>, content_type = "application/json"),
    ),
    tag = crate::api::USERS_TAG
)]
pub async fn get_users(
    Extension(db): Extension<PgPool>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<Vec<User>>> {
    Ok(Json(
        DbUser::all(&db, query.page_size, query.page - 1).await?,
    ))
}
