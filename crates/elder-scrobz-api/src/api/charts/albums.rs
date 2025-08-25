use crate::api::charts::ChartQuery;
use crate::api::pagination::ToOffset;
use crate::api::PaginatedResponse;
use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::get_most_listened_albums;
use elder_scrobz_db::PgPool;
use elder_scrobz_model::album::ChartAlbum;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/albums",
    summary = "Album charts",
    params(ChartQuery),
    responses(
        (status = 200, description = "Top albums", body = PaginatedResponse<ChartAlbum>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn album_charts(
    State(db): State<PgPool>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<PaginatedResponse<ChartAlbum>>> {
    let offset = query.to_offset();
    let (total, albums) =
        get_most_listened_albums(query.period, query.username, query.page_size, offset, &db)
            .await?;

    let albums: Vec<_> = albums.into_iter().map(ChartAlbum::from).collect();

    let response = PaginatedResponse {
        data: albums,
        page: query.page,
        page_size: query.page_size,
        total,
    };

    Ok(Json(response))
}
