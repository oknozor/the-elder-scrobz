use crate::api::charts::ChartQuery;
use crate::api::pagination::ToOffset;
use crate::api::PaginatedResponse;
use crate::error::{AppError, AppResult};
use crate::settings::Settings;
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::charts::artists::{get_most_listened_artists, TopArtist};
use elder_scrobz_db::{PgPool, WithLocalImage};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug)]
#[serde(tag = "type")]
pub enum Artist {
    Artist(TopArtist),
}

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
    State(db): State<PgPool>,
    Extension(settings): Extension<Arc<Settings>>,
) -> AppResult<Json<PaginatedResponse<Artist>>> {
    let offset = query.to_offset();
    let (total, artists) =
        get_most_listened_artists(query.period, query.username, query.page_size, offset, &db)
            .await?;

    let artists: Vec<_> = artists
        .into_iter()
        .map(|artist| artist.with_local_image(&settings.coverart_path))
        .map(Artist::Artist)
        .collect();

    let response = PaginatedResponse {
        data: artists,
        page: query.page,
        page_size: query.page_size,
        total,
    };
    Ok(Json(response))
}
