use crate::error::AppResult;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::user::CreateUser;

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> AppResult<()> {
    user.insert(&state.pool).await?;
    Ok(())
}
