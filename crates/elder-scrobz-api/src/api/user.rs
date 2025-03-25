use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::tracks::{get_most_listened_track, TopTrack};
use elder_scrobz_db::charts::Period;
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

#[derive(Deserialize, Debug)]
pub struct ChartQuery {
    period: Period,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/users/{id}/charts/tracks",
    responses(
        (status = 200, description = "Top tracks for user", body = Vec<TopTrack>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn top_tracks(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopTrack>>> {
    let tracks = get_most_listened_track(query.period, &state.pool)
        .await?
        .into_iter()
        .map(TopTrack::from)
        .collect();

    Ok(Json(tracks))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/users/{id}/charts/tracks",
    responses(
        (status = 200, description = "Top album for user", body = Vec<TopTrack>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn top_albums(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> AppResult<Json<Vec<TopTrack>>> {
    todo!()
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/users/{id}/charts/tracks",
    responses(
        (status = 200, description = "Top artists for user", body = Vec<TopTrack>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn top_artists(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> AppResult<Json<Vec<TopTrack>>> {
    todo!()
}
