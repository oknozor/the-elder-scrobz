use crate::api::imports::*;
use crate::oauth::AuthenticatedUser;
use axum::middleware::from_extractor;
use serde::Serialize;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme, HttpBuilder, HttpAuthScheme};
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod admin;
pub mod charts;
pub mod imports;
pub mod listenbrainz;
pub mod listens;
pub mod pagination;
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
        schemas(elder_scrobz_db::listens::recent::RecentListen, pagination::PageQuery, crate::error::AppError)
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build()
                ),
            )
        }
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    page: i64,
    page_size: i64,
    total: i64,
}

pub fn router(no_oauth: bool) -> OpenApiRouter {
    let mut router = OpenApiRouter::new()
        .routes(routes!(import_listens))
        .nest("/users", user::router())
        .nest("/charts", charts::router())
        .nest("/admin", admin::router())
        .nest("/listens", listens::router());

    if !no_oauth {
        router = router.layer(from_extractor::<AuthenticatedUser>())
    }

    router.merge(listenbrainz::router())
}
