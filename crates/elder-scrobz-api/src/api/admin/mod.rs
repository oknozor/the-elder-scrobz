use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

mod config;
mod releases;
mod scan;
mod scrobbles;
mod stats;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(scan::scan_db))
        .routes(routes!(stats::stats))
        .routes(routes!(releases::remove_coverart))
        .routes(routes!(config::get_config))
        .routes(routes!(config::set_config))
        .merge(scrobbles::router())
}
