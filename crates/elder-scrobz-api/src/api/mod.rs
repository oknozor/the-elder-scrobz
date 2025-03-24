use crate::api::api_key::create_api_key;
use crate::api::listens::{import_listens, submit_listens};
use crate::api::user::{create_user, top_tracks};
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;

pub mod api_key;
pub mod listens;
pub mod user;

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/1/submit-listens", post(submit_listens))
        .route("/users", post(create_user))
        .route("/users/{id}/import", post(import_listens))
        .route("/users/{id}/api-key/create", post(create_api_key))
        .route("/users/{id}/stats/tracks", get(top_tracks))
}
