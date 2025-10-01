use crate::api::charts::ChartQuery;
use crate::api::pagination::{PaginatedResponse, ToOffset};
use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::tracks::get_most_listened_tracks;
use elder_scrobz_db::PgPool;
use elder_scrobz_model::track::ChartTrack;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/tracks",
    summary = "Track charts",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top tracks for user", body = PaginatedResponse<ChartTrack>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn track_charts(
    Query(query): Query<ChartQuery>,
    State(db): State<PgPool>,
) -> AppResult<Json<PaginatedResponse<ChartTrack>>> {
    let offset = query.to_offset();
    let (total, tracks) = get_most_listened_tracks(
        query.period,
        query.username.as_ref(),
        query.page_size,
        offset,
        &db,
    )
    .await?;

    let tracks: Vec<_> = tracks.into_iter().map(ChartTrack::from).collect();
    let response = PaginatedResponse::from_query(tracks, total, query);

    Ok(Json(response))
}
