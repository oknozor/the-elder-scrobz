use crate::settings::AuthSettings;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::{
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use http::HeaderValue;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub sub: String,
    pub email: Option<String>,
}

pub async fn verify_bearer_token(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let headers = req.headers();
    if let Some(auth_value) = extract_bearer_token(&headers) {
        match validate_token(&auth_value, &state.settings.oauth_provider).await {
            Ok(authenticated_user) => {
                req.extensions_mut().insert(authenticated_user);
                next.run(req).await
            }
            Err(_) => axum::response::Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::from("Unauthorized"))
                .unwrap(),
        }
    } else {
        axum::response::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::from("Unauthorized"))
            .unwrap()
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|header: &HeaderValue| header.to_str().ok())
        .and_then(|value: &str| value.strip_prefix("Bearer "))
        .map(|token: &str| token.to_string())
}

async fn validate_token(token: &str, auth_config: &AuthSettings) -> Result<AuthenticatedUser, ()> {
    let client = Client::new();
    let response = client
        .post(&auth_config.introspection_url)
        .send()
        .await
        .map_err(|_| ())?;

    if response.status().is_success() {
        let token_info: TokenIntrospectionResponse = response.json().await.map_err(|_| ())?;
        if token_info.active {
            Ok(AuthenticatedUser {
                sub: token_info.sub.clone(),
                email: token_info.email.clone(),
            })
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

#[derive(Debug, Deserialize)]
struct TokenIntrospectionResponse {
    active: bool,
    sub: String,
    email: Option<String>,
}
