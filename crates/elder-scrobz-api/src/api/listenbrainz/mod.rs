use api_key::*;
use axum::http::header::ToStrError;
use axum::http::HeaderValue;
use axum_extra::headers::authorization::Credentials;
use listens::*;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

mod api_key;
mod listens;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(submit_listens))
        .routes(routes!(create_api_key))
        .routes(routes!(get_api_keys))
        .routes(routes!(delete_api_key))
        .routes(routes!(validate_token))
}

pub struct Token(HeaderValue);

impl Credentials for Token {
    const SCHEME: &'static str = "Token";

    fn decode(value: &HeaderValue) -> Option<Self> {
        Token(value.clone()).into()
    }

    fn encode(&self) -> HeaderValue {
        (&self.0).into()
    }
}

impl Token {
    pub fn token(&self) -> Result<Option<&str>, ToStrError> {
        let token = self.0.to_str()?;
        Ok(token.strip_prefix("Token "))
    }
}
