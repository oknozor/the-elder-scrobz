use crate::api::charts::ChartQuery;
use crate::api::pagination::ToOffset;
use crate::api::PaginatedResponse;
use crate::error::{AppError, AppResult};
use crate::settings::Settings;
use autometrics::autometrics;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::charts::artists::{get_most_listened_artists, TopArtist};
use elder_scrobz_db::PgPool;
use std::sync::Arc;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/artists",
    summary = "Artist charts",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top album for user", body = Vec<TopArtist>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn artist_charts(
    Query(query): Query<ChartQuery>,
    Extension(db): Extension<PgPool>,
    Extension(settings): Extension<Arc<Settings>>,
) -> AppResult<Json<PaginatedResponse<TopArtist>>> {
    let offset = query.to_offset();
    let (total, artists) =
        get_most_listened_artists(query.period, query.username, query.page_size, offset, &db)
            .await?;

    let artists: Vec<_> = artists
        .into_iter()
        .map(|mut artist| {
            if let Some(th) = settings.coverart_url(&artist.artist_id) {
                artist.thumbnail_url = Some(th)
            }

            artist
        })
        .collect();

    let response = PaginatedResponse {
        data: artists,
        page: query.page,
        page_size: query.page_size,
        total,
    };
    Ok(Json(response))
}
