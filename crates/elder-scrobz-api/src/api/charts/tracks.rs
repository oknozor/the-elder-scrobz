use crate::api::charts::ChartQuery;
use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::tracks::{get_most_listened_tracks, TopTrack};

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
    let tracks = get_most_listened_tracks(query.period, query.username, &state.pool).await?;
    let tracks: Vec<_> = tracks
        .into_iter()
        .map(|mut track| {
            if let Some(ca) = state.settings.coverart_url(&track.release_mbid) {
                track.cover_art_url = Some(ca)
            }

            track
        })
        .collect();
    Ok(Json(tracks))
}
