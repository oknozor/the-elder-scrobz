use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::{get_most_listened_albums, TopAlbum};
use elder_scrobz_db::charts::tracks::{get_most_listened_tracks, TopTrack};
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

#[derive(Deserialize, ToSchema, Debug)]
pub struct ChartQuery {
    period: Period,
    user_id: Option<String>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/users/{id}/charts/tracks",
    params(
        ("id" = String, Path, description = "ID of the user"),
        ("params" = ChartQuery, description = "Chart period such as 'year', 'month', 'week', or 'today'")
    ),
    responses(
        (status = 200, description = "Top tracks for user", body = Vec<TopTrack>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn track_charts(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopTrack>>> {
    Ok(Json(
        get_most_listened_tracks(query.period, query.user_id, &_state.pool).await?,
    ))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/users/{id}/charts/albums",
    params(
        ("id" = String, Path, description = "ID of the user"),
        ("period" = ChartQuery, description = "Chart period such as 'year', 'month', 'week', or 'today'")
    ),
    responses(
        (status = 200, description = "Top album for user", body = Vec<TopAlbum>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn album_charts(
    State(_state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopAlbum>>> {
    Ok(Json(
        get_most_listened_albums(query.period, query.user_id, &_state.pool).await?,
    ))
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
pub async fn _top_artists(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
) -> AppResult<Json<Vec<TopTrack>>> {
    todo!()
}
