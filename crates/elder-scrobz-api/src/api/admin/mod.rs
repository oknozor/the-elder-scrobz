use crate::AppState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod scan;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(scan::scan_db))
}
