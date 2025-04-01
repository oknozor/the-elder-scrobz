use crate::api::charts::ChartQuery;
use crate::api::pagination::ToOffset;
use crate::api::PaginatedResponse;
use crate::error::{AppError, AppResult};
use crate::settings::Settings;
use autometrics::autometrics;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::charts::tracks::{get_most_listened_tracks, TopTrack};
use elder_scrobz_db::PgPool;
use std::sync::Arc;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/tracks",
    summary = "Track charts",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top tracks for user", body = PaginatedResponse<TopTrack>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn track_charts(
    Query(query): Query<ChartQuery>,
    Extension(db): Extension<PgPool>,
    Extension(settings): Extension<Arc<Settings>>,
) -> AppResult<Json<PaginatedResponse<TopTrack>>> {
    let offset = query.to_offset();
    let (total, tracks) =
        get_most_listened_tracks(query.period, query.username, query.page_size, offset, &db)
            .await?;

    let tracks: Vec<_> = tracks
        .into_iter()
        .map(|mut track| {
            if let Some(ca) = settings.coverart_url(&track.release_mbid) {
                track.cover_art_url = Some(ca)
            }

            track
        })
        .collect();

    let response = PaginatedResponse {
        data: tracks,
        page: query.page,
        page_size: query.page_size,
        total,
    };

    Ok(Json(response))
}
