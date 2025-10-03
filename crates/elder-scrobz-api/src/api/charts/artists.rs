use crate::api::charts::ChartQuery;
use crate::api::pagination::{PaginatedResponse, ToOffset};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::artists::get_most_listened_artists;
use elder_scrobz_model::artist::ChartArtist;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/artists",
    summary = "Artist charts",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top album for user", body = Vec<ChartArtist>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn artist_charts(
    Query(query): Query<ChartQuery>,
    State(state): State<AppState>,
) -> AppResult<Json<PaginatedResponse<ChartArtist>>> {
    let offset = query.to_offset();
    let (total, artists) = get_most_listened_artists(
        query.period,
        query.username.as_ref(),
        query.page_size,
        offset,
        &state.db,
    )
    .await?;
    let artists: Vec<_> = artists.into_iter().map(ChartArtist::from).collect();
    let response = PaginatedResponse::from_query(artists, total, query);
    Ok(Json(response))
}
