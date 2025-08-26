use crate::api::charts::ChartQuery;
use crate::api::PaginatedResponse;
use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::album::TopAlbum;
use elder_scrobz_db::charts::overview::{get_overview, Overview};
use elder_scrobz_db::PgPool;

#[debug_handler]
#[utoipa::path(
    get,
    path = "/overview",
    summary = "Listening overview",
    params(ChartQuery),
    responses(
        (status = 200, description = "Overview", body = PaginatedResponse<TopAlbum>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn overview(
    State(db): State<PgPool>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Overview>> {
    Ok(Json(
        get_overview(query.period, query.username, &db)
            .await?
            .unwrap_or_default(),
    ))
}
