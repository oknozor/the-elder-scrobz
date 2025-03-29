use crate::api::charts::ChartQuery;
use crate::error::{AppError, AppResult};
use crate::settings::Settings;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::{get_most_listened_albums, TopAlbum};
use elder_scrobz_db::PgPool;
use std::sync::Arc;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/albums",
    summary = "Album charts",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top albums", body = Vec<TopAlbum>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
pub async fn album_charts(
    Extension(db): Extension<PgPool>,
    Extension(settings): Extension<Arc<Settings>>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Vec<TopAlbum>>> {
    let albums = get_most_listened_albums(
        query.period,
        query.username,
        query.page,
        query.page_size,
        &db,
    )
    .await?;

    let albums: Vec<_> = albums
        .into_iter()
        .map(|mut album| {
            if let Some(ca) = settings.coverart_url(&album.release_id) {
                album.cover_art_url = Some(ca)
            }

            album
        })
        .collect();

    Ok(Json(albums))
}
