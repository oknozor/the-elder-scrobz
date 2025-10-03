use crate::api::pagination::{PageQuery, PaginatedResponse, ToOffset};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::recent::{get_recent_listens, RecentListen};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(Serialize, ToSchema, Debug)]
#[serde(tag = "type")]
pub enum RecentTrack {
    Track(RecentListen),
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(recent_listens))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/recent",
    summary = "Recent listens",
    params(PageQuery),
    responses(
        (status = 200, description = "Recent tracks listened", body = PaginatedResponse<RecentTrack>, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = crate::api::SCROBBLES_TAG
)]
#[autometrics]
pub async fn recent_listens(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<PaginatedResponse<RecentTrack>>> {
    let (total, listens) =
        get_recent_listens(query.per_page(), query.to_offset(), &state.db).await?;

    let listens = listens
        .into_iter()
        .map(RecentTrack::Track)
        .collect::<Vec<_>>();

    let response = PaginatedResponse::from_query(listens, total, query);

    Ok(Json(response))
}
