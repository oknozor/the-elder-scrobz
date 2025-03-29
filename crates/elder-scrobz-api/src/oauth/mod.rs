use crate::error::AppError;
use crate::oauth::client::Oauth2Client;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderMap;
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

impl FromRequestParts<()> for AuthenticatedUser {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _: &()) -> Result<Self, Self::Rejection> {
        let oauth2_client = parts
            .extensions
            .get::<Oauth2Client>()
            .expect("Missing Oauth2Client extension");
        match extract_bearer_token(&parts.headers) {
            Some(auth_value) => match oauth2_client.introspect(auth_value).await {
                Ok(response) => match AuthenticatedUser::from_introspection(response) {
                    None => Err(AppError::Unauthorized(
                        "Token is invalid or inactive".to_string(),
                    )),
                    Some(user) => Ok(user),
                },
                Err(err) => Err(AppError::Unauthorized(err.to_string())),
            },
            None => Err(AppError::Unauthorized("Missing Bearer Token".to_string())),
        }
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("authorization")?;
    let bytes = header.as_bytes();
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => value
            .strip_prefix("Bearer ")
            .map(|token: &str| token.to_string()),
        Err(_) => None,
    }
}
