use crate::api::charts::albums::*;
use crate::api::charts::artists::*;
use crate::api::charts::overview::__path_overview;
use crate::api::charts::overview::overview;
use crate::api::charts::pulses::*;
use crate::api::charts::tracks::*;
use crate::api::pagination::ToOffset;
use elder_scrobz_db::Period;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod albums;
pub mod artists;
pub mod overview;
pub mod pulses;
pub mod tracks;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(track_charts))
        .routes(routes!(album_charts))
        .routes(routes!(artist_charts))
        .routes(routes!(pulses))
        .routes(routes!(overview))
}

#[derive(Deserialize, IntoParams, Debug)]
#[serde(default)]
pub struct ChartQuery {
    // Year | month | week | today
    period: Period,
    // The username to filter result on
    username: Option<String>,
    // Page to query
    page: i64,
    // Number of item in a page
    page_size: i64,
}

impl ToOffset for ChartQuery {
    fn page(&self) -> i64 {
        self.page
    }

    fn per_page(&self) -> i64 {
        self.page_size
    }
}

impl Default for ChartQuery {
    fn default() -> Self {
        ChartQuery {
            period: Period::Year,
            username: None,
            page: 1,
            page_size: 15,
        }
    }
}
