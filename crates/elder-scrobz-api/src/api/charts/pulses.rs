use crate::error::{AppError, AppResult};
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::pulses::Pulse;
use elder_scrobz_db::{Period, PgPool};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams, Debug)]
pub struct PulseQuery {
    period: Period,
    user_id: Option<String>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/pulses",
    params(PulseQuery),
    responses(
        (status = 200, description = "Top album for user", body = Vec<Pulse>, content_type = "application/json"),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::CHARTS_TAG
)]
pub async fn pulses(
    Query(query): Query<PulseQuery>,
    Extension(db): Extension<PgPool>,
) -> AppResult<Json<Vec<Pulse>>> {
    Ok(Json(
        Pulse::for_period(query.period, query.user_id, &db).await?,
    ))
}
