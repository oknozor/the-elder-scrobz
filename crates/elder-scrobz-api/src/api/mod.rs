use crate::api::imports::*;
use crate::AppState;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{IntoParams, Modify, OpenApi, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod admin;
pub mod charts;
pub mod imports;
pub mod listenbrainz;
pub mod listens;
pub mod user;

const USERS_TAG: &str = "users";
const CHARTS_TAG: &str = "charts";
const SCROBBLES_TAG: &str = "scrobbles";
const API_KEYS_TAG: &str = "apikey";
const ADMIN_TAG: &str = "admin";

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = USERS_TAG, description = "Users"),
        (name = CHARTS_TAG, description = "Charts"),
        (name = SCROBBLES_TAG, description = "Scrobbles"),
        (name = API_KEYS_TAG, description = "ApiKey"),
        (name = ADMIN_TAG, description = "Administration"),
    ),
    components(
        schemas(elder_scrobz_db::listens::recent::RecentListen, PageQuery, crate::error::AppError)
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

#[derive(serde::Deserialize, ToSchema, IntoParams, Debug)]
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
    let api = OpenApiRouter::new().routes(routes!(import_listens));

    listenbrainz::router()
        .merge(api)
        .nest("/users", user::router())
        .nest("/charts", charts::router())
        .nest("/admin", admin::router())
        .nest("/listens", listens::router())
}
