use crate::error::AppResult;
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::scrobble;
use elder_scrobz_db::scrobble::Listen;
use elder_scrobz_db::user::{CreateUser, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreated {
    pub(crate) user_id: String,
}

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> AppResult<Json<UserCreated>> {
    let uuid = user.insert(&state.pool).await?;
    Ok(Json(UserCreated {
        user_id: uuid.to_string(),
    }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TracksStat {
    pub user_id: String,
    pub scrobbles: Vec<Listen>,
}

#[debug_handler]
pub async fn top_tracks(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> AppResult<Json<TracksStat>> {
    let scrobbles = scrobble::fetch_scrobbles_for_user(&state.pool, &user_id).await?;

    Ok(Json(TracksStat { user_id, scrobbles }))
}
