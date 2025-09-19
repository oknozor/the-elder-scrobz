use elder_scrobz_db::PgPool;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod releases;
mod scan;
mod scrobbles;
mod stats;

pub fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(scan::scan_db))
        .routes(routes!(stats::stats))
        .routes(routes!(releases::remove_coverart))
        .merge(scrobbles::router())
}
