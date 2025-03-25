use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use elder_scrobz_db::charts::album::TopAlbum;

#[derive(Deserialize, ToSchema, Debug)]
pub struct PulseQuery {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    user: Option<String>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/users/{id}/charts/artists",
    params(
        ("id" = String, Path, description = "ID of the user"),
        ("period" = PulseQuery, description = "Chart period such as 'year', 'month', 'week', or 'today'")
    ),
    responses(
        (status = 200, description = "Top album for user", body = Vec<TopAlbum>),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn pulses(
    State(_state): State<AppState>,
    Query(query): Query<PulseQuery>,
) -> AppResult<Json<Vec<()>>> {
    todo!()
}
