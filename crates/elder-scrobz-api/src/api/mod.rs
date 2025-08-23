use crate::oauth::AuthenticatedUser;
use axum::middleware::from_extractor;
use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_axum::router::OpenApiRouter;

pub mod admin;
pub mod albums;
pub mod artists;
pub mod charts;
pub mod listenbrainz;
pub mod listens;
pub mod pagination;
pub mod tracks;
pub mod users;

const USERS_TAG: &str = "users";
const CHARTS_TAG: &str = "charts";
const SCROBBLES_TAG: &str = "scrobbles";
const API_KEYS_TAG: &str = "apikey";
const ADMIN_TAG: &str = "admin";
const TRACKS_TAG: &str = "tracks";
const ALBUMS_TAG: &str = "album";
const ARTISTS_TAG: &str = "artists";

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = USERS_TAG, description = "Users"),
        (name = CHARTS_TAG, description = "Charts"),
        (name = SCROBBLES_TAG, description = "Scrobbles"),
        (name = API_KEYS_TAG, description = "ApiKey"),
        (name = ADMIN_TAG, description = "Administration"),
        (name = TRACKS_TAG, description = "Tracks"),
        (name = ALBUMS_TAG, description = "Album"),
        (name = ARTISTS_TAG, description = "Artists"),
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
                        .build(),
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
        .nest("/users", users::router())
        .nest("/charts", charts::router())
        .nest("/admin", admin::router())
        .nest("/listens", listens::router())
        .nest("/tracks", tracks::router())
        .nest("/albums", albums::router())
        .nest("/artists", artists::router());

    if !no_oauth {
        router = router.layer(from_extractor::<AuthenticatedUser>())
    }

    router.merge(listenbrainz::router())
}
