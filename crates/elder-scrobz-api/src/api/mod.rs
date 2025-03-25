use crate::api::charts::*;
use crate::api::imports::*;
use crate::AppState;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod charts;
pub mod imports;
pub mod listenbrainz;
pub mod user;

#[derive(serde::Deserialize, ToSchema, Debug)]
#[serde(default)]
pub struct PageQuery {
    page: i64,
    page_size: i64,
}

impl Default for PageQuery {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 100,
        }
    }
}

pub fn router() -> OpenApiRouter<AppState> {
    let api = OpenApiRouter::new()
        .routes(routes!(import_listens))
        .routes(routes!(pulses));

    listenbrainz::router()
        .merge(api)
        .nest("/users", user::router())
        .nest("/charts", charts::router())
}
