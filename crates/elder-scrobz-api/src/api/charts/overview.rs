use crate::api::charts::ChartQuery;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::charts::overview::{get_overview, Overview};

#[debug_handler]
#[utoipa::path(
    get,
    path = "/overview",
    summary = "Listening overview",
    params(ChartQuery),
    responses(
        (status = 200, description = "Overview", body = Overview, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
#[autometrics]
pub async fn overview(
    State(state): State<AppState>,
    Query(query): Query<ChartQuery>,
) -> AppResult<Json<Overview>> {
    Ok(Json(
        get_overview(query.period, query.username, &state.db)
            .await?
            .unwrap_or_default(),
    ))
}
