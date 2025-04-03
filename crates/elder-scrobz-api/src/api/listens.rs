use crate::api::pagination::{PageQuery, ToOffset};
use crate::api::PaginatedResponse;
use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_db::listens::recent::{get_recent_listens, RecentListen};
use elder_scrobz_db::PgPool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(recent_listens))
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/recent",
    summary = "Recent listens",
    params(PageQuery),
    responses(
        (status = 200, description = "Recent tracks listened", body = PaginatedResponse<RecentListen>, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = crate::api::SCROBBLES_TAG
)]
#[autometrics]
pub async fn recent_listens(
    Extension(db): Extension<PgPool>,
    Query(query): Query<PageQuery>,
) -> AppResult<Json<PaginatedResponse<RecentListen>>> {
    let (total, listens) = get_recent_listens(query.per_page(), query.to_offset(), &db).await?;

    let response = PaginatedResponse {
        data: listens,
        page: query.page(),
        page_size: query.per_page(),
        total,
    };

    Ok(Json(response))
}
