use crate::api::charts::ChartQuery;
use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::artists::{get_most_listened_artists, TopArtist};

#[debug_handler]
#[utoipa::path(
    get,
    path = "/artists",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top album for user", body = Vec<TopArtist>, content_type = "application/json"),
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
