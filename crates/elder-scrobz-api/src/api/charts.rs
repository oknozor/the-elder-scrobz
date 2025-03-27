use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::{get_most_listened_albums, TopAlbum};
use elder_scrobz_db::charts::artists::{get_most_listened_artists, TopArtist};
use elder_scrobz_db::charts::tracks::{get_most_listened_tracks, TopTrack};
use elder_scrobz_db::pulses::Pulse;
use elder_scrobz_db::Period;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(track_charts))
        .routes(routes!(album_charts))
        .routes(routes!(artist_charts))
        .routes(routes!(pulses))
}

#[derive(Deserialize, IntoParams, Debug)]
#[serde(default)]
pub struct ChartQuery {
    // Year | month | week | today
    period: Period,
    // The username to filter result on
    username: Option<String>,
    // Page to query
    page: i64,
    // Number of item in a page
    page_size: i64,
}

impl Default for ChartQuery {
    fn default() -> Self {
        ChartQuery {
            period: Period::Year,
            username: None,
            page: 1,
            page_size: 15,
        }
    }
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/tracks",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top tracks for user", body = Vec<TopTrack>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
pub async fn track_charts(
    State(state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopTrack>>> {
    Ok(Json(
        get_most_listened_tracks(query.period, query.username, &state.pool).await?,
    ))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/albums",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top albums", body = Vec<TopAlbum>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
pub async fn album_charts(
    State(state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopAlbum>>> {
    Ok(Json(
        get_most_listened_albums(
            query.period,
            query.username,
            query.page,
            query.page_size,
            &state.pool,
        )
        .await?,
    ))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/artists",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top album for user", body = Vec<TopAlbum>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
pub async fn artist_charts(
    State(state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopArtist>>> {
    Ok(Json(
        get_most_listened_artists(query.period, query.username, &state.pool).await?,
    ))
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct PulseQuery {
    period: Period,
    user_id: Option<String>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/pulses",
    params(PulseQuery),
    responses(
        (status = 200, description = "Top album for user", body = Vec<Pulse>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
pub async fn pulses(
    State(state): State<AppState>,
    Query(query): Query<PulseQuery>,
) -> AppResult<Json<Vec<Pulse>>> {
    Ok(Json(
        Pulse::for_period(query.period, query.user_id, &state.pool).await?,
    ))
}
