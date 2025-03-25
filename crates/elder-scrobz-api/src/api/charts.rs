use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::{get_most_listened_albums, TopAlbum};
use elder_scrobz_db::charts::tracks::{get_most_listened_tracks, TopTrack};
use elder_scrobz_db::Period;
use serde::Deserialize;
use utoipa::ToSchema;

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
    path = "/users/{id}/charts/artists",
    params(
        ("id" = String, Path, description = "ID of the user"),
        ("period" = ChartQuery, description = "Chart period such as 'year', 'month', 'week', or 'today'")
    ),
    responses(
        (status = 200, description = "Top album for user", body = Vec<TopAlbum>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn artist_charts(
    State(_state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopAlbum>>> {
    Ok(Json(
        get_most_listened_albums(query.period, query.user_id, &_state.pool).await?,
    ))
}
