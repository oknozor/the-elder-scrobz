use crate::api::api_key::create_api_key;
use crate::api::listens::{import_listens, submit_listens};
use crate::api::user::create_user;
use crate::AppState;
use axum::routing::post;
use axum::Router;

mod api_key;
mod listens;
mod user;

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/1/submit-listens", post(submit_listens))
        .route("/users", post(create_user))
        .route("/users/{id}/import", post(import_listens))
        .route("/users/{id}/api-key/create", post(create_api_key))
}
