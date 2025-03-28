use crate::api::charts::ChartQuery;
use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::{get_most_listened_albums, TopAlbum};

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
    let albums = get_most_listened_albums(
        query.period,
        query.username,
        query.page,
        query.page_size,
        &state.pool,
    )
    .await?;

    let albums: Vec<_> = albums
        .into_iter()
        .map(|mut album| {
            if let Some(ca) = state.settings.coverart_url(&album.release_id) {
                album.cover_art_url = Some(ca)
            }

            album
        })
        .collect();

    Ok(Json(albums))
}
