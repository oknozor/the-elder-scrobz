use crate::error::AppResult;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::user::CreateUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserCreated {
    pub(crate) user_id: String,
}
#[debug_handler]
#[utoipa::path(
    post,
    path = "/users",
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
