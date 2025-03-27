use crate::state::AppState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod scan;
mod scrobbles;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(scan::scan_db))
        .routes(routes!(scrobbles::get_by_id))
}
