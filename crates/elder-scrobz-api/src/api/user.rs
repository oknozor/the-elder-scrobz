use crate::api::PageQuery;
use crate::error::AppResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::user::{CreateUser, User as DbUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(get_users))
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserCreated {
    pub(crate) user_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
}

impl From<DbUser> for User {
    fn from(user: DbUser) -> Self {
        Self {
            username: user.username,
            email: user.email,
        }
    }
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = 200, description = "User Created", body = UserCreated),
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> AppResult<Json<UserCreated>> {
    let uuid = user.insert(&state.pool).await?;
    Ok(Json(UserCreated {
        user_id: uuid.to_string(),
    }))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "All users", body = UserCreated),
    )
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<Vec<User>>> {
    let users = DbUser::all(&state.pool, query.page_size, query.page - 1)
        .await?
        .into_iter()
        .map(User::from)
        .collect();

    Ok(Json(users))
}
