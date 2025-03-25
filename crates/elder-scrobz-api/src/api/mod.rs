use crate::api::charts::*;
use crate::api::imports::*;
use crate::api::pulse::*;
use crate::api::user::*;
use crate::AppState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod charts;
pub mod imports;
pub mod listenbrainz;
pub mod pulse;
pub mod user;

pub fn router() -> OpenApiRouter<AppState> {
    let api = OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(import_listens))
        .routes(routes!(track_charts))
        .routes(routes!(album_charts))
        .routes(routes!(artist_charts))
        .routes(routes!(pulses));

    listenbrainz::router().nest("/api/v1/", api)
}
