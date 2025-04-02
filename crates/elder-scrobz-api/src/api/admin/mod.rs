use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod releases;
mod scan;
mod scrobbles;
mod stats;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(scan::scan_db))
        .routes(routes!(stats::stats))
        .routes(routes!(scrobbles::get_by_id))
        .routes(routes!(releases::remove_coverart))
}
