use crate::api::api_key::*;
use crate::api::listens::*;
use crate::api::user::*;
use crate::AppState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod api_key;
pub mod listens;
pub mod user;

pub fn app() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(submit_listens))
        .routes(routes!(create_user))
        .routes(routes!(import_listens))
        .routes(routes!(create_api_key))
        .routes(routes!(top_tracks))
    // .routes(routes!(top_albums))
    // .routes(routes!(top_artists))
}
