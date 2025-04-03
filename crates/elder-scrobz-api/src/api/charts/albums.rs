use crate::api::charts::ChartQuery;
use crate::api::pagination::ToOffset;
use crate::api::PaginatedResponse;
use crate::error::{AppError, AppResult};
use crate::settings::Settings;
use autometrics::autometrics;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::{get_most_listened_albums, TopAlbum};
use elder_scrobz_db::{PgPool, WithLocalImage};
use std::sync::Arc;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/albums",
    summary = "Album charts",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top albums", body = PaginatedResponse<TopAlbum>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn album_charts(
    Extension(db): Extension<PgPool>,
    Extension(settings): Extension<Arc<Settings>>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<PaginatedResponse<TopAlbum>>> {
    let offset = query.to_offset();
    let (total, albums) =
        get_most_listened_albums(query.period, query.username, query.page_size, offset, &db)
            .await?;

    let albums: Vec<_> = albums
        .into_iter()
        .map(|album| album.with_local_image(&settings.coverart_path))
        .collect();

    let response = PaginatedResponse {
        data: albums,
        page: query.page,
        page_size: query.page_size,
        total,
    };

    Ok(Json(response))
}
