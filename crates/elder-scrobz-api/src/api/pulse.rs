use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::pulses::Pulse;
use elder_scrobz_db::Period;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct PulseQuery {
    period: Period,
    user_id: Option<String>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/pulses",
    params(
        ("id" = String, Path, description = "ID of the user"),
        ("period" = Period, description = "Chart period such as 'year', 'month', 'week', or 'today'"),
        ("user_id" = Option<String>, description = "filter by user id")
    ),
    responses(
        (status = 200, description = "Top album for user", body = Vec<Pulse>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn pulses(
    State(_state): State<AppState>,
    Query(query): Query<PulseQuery>,
) -> AppResult<Json<Vec<Pulse>>> {
    Ok(Json(
        Pulse::for_period(query.period, query.user_id, &_state.pool).await?,
    ))
}
