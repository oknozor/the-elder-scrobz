use crate::api::pagination::{PageQuery, PaginatedResponse, ToOffset};
use crate::error::AppResult;
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::user::{CreateUser, User as DbUser, User};
use elder_scrobz_db::PgPool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod exports;
pub mod imports;

pub fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(get_users))
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
    State(db): State<PgPool>,
    Json(user): Json<CreateUser>,
) -> AppResult<Json<User>> {
    Ok(Json(user.insert(&db).await?))
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
    State(db): State<PgPool>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<PaginatedResponse<User>>> {
    let (total, users) = DbUser::all(&db, query.per_page(), query.to_offset()).await?;
    let response = PaginatedResponse::from_query(users, total, query);

    Ok(Json(response))
}
