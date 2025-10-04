use crate::error::AppError;
use crate::oauth::client::Oauth2Client;
use crate::state::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderMap;
use elder_scrobz_db::user::{CreateUser, User};
use oauth2::basic::BasicTokenIntrospectionResponse;
use oauth2::TokenIntrospectionResponse;

pub mod client;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub name: String,
}

impl AuthenticatedUser {
    fn from_introspection(value: BasicTokenIntrospectionResponse) -> Option<AuthenticatedUser> {
        value.active().then(|| AuthenticatedUser {
            name: value
                .sub()
                .expect("no sub claim in token introspection")
                .to_string(),
        })
    }
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let oauth2_client = parts
            .extensions
            .get::<Oauth2Client>()
            .expect("Missing Oauth2Client extension");

        let auth_value = bearer_from_headers(&parts.headers).or_else(|| match parts.uri.query() {
            Some(query) => bearer_from_query_params(query),
            None => None,
        });

        match auth_value {
            Some(token) => match oauth2_client.introspect(token).await {
                Ok(response) => match AuthenticatedUser::from_introspection(response) {
                    None => Err(AppError::Unauthorized(
                        "Token is invalid or inactive".to_string(),
                    )),
                    Some(user) => {
                        let existing_user = User::get_by_username(&state.db, &user.name).await?;
                        if existing_user.is_none() {
                            CreateUser {
                                username: user.name.clone(),
                            }
                            .insert(&state.db)
                            .await?;
                        }
                        Ok(user)
                    }
                },
                Err(err) => Err(AppError::Unauthorized(err.to_string())),
            },
            None => Err(AppError::Unauthorized("Missing Bearer Token".to_string())),
        }
    }
}

fn bearer_from_query_params(query: &str) -> Option<String> {
    let params: std::collections::HashMap<String, String> =
        url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();

    params.get("token").cloned()
}

fn bearer_from_headers(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("authorization")?;
    let bytes = header.as_bytes();
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => value
            .strip_prefix("Bearer ")
            .map(|token: &str| token.to_string()),
        Err(_) => None,
    }
}
