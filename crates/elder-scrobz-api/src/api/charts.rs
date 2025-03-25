use crate::api::PageQuery;
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
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(track_charts))
        .routes(routes!(album_charts))
        .routes(routes!(artist_charts))
        .routes(routes!(pulses))
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct ChartQuery {
    period: Period,
    username: Option<String>,
    #[serde(flatten)]
    page: PageQuery,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/tracks",
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
    State(state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopAlbum>>> {
    Ok(Json(
        get_most_listened_albums(query.period, query.username, &state.pool).await?,
    ))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/artists",
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
    State(state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopArtist>>> {
    Ok(Json(
        get_most_listened_artists(query.period, query.username, &state.pool).await?,
    ))
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct PulseQuery {
    period: Period,
    user_id: Option<String>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/pulses",
    params(
        ("id" = String, Path, description = "ID of the user"),
        ("period" = Period, description = "Chart period such as 'year', 'month', 'week', or 'today'"),
        ("user_id" = Option<String>, description = "filter by user id")
    ),
    responses(
        (status = 200, description = "Top album for user", body = Vec<Pulse>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn pulses(
    State(state): State<AppState>,
    Query(query): Query<PulseQuery>,
) -> AppResult<Json<Vec<Pulse>>> {
    Ok(Json(
        Pulse::for_period(query.period, query.user_id, &state.pool).await?,
    ))
}
