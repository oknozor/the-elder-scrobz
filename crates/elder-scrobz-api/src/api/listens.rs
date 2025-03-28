use crate::api::PageQuery;
use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::recent::{get_recent_listens, RecentListen};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(recent_listens))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/recent",
    params(PageQuery),
    responses(
        (status = 200, description = "Recent tracks listened", body = Vec<RecentListen>, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = crate::api::SCROBBLES_TAG
)]
pub async fn recent_listens(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<Vec<RecentListen>>> {
    Ok(Json(
        get_recent_listens(query.page, query.page_size, &state.pool).await?,
    ))
}
