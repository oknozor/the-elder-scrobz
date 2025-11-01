use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_session_auth::Authentication;
use elder_scrobz_db::{user::User, PgPool};
use reqwest::StatusCode;

use crate::oauth::router::Session;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub username: String,
}

#[async_trait]
impl Authentication<AuthenticatedUser, String, PgPool> for AuthenticatedUser {
    async fn load_user(
        username: String,
        pool: Option<&PgPool>,
    ) -> Result<AuthenticatedUser, anyhow::Error> {
        let pool = pool.unwrap();

        User::get_by_username(pool, &username)
            .await?
            .map(|user| AuthenticatedUser {
                username: user.username,
            })
            .ok_or_else(|| anyhow::anyhow!("User not found"))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "No session found".to_string()))?;

        session.current_user.ok_or((
            StatusCode::UNAUTHORIZED,
            "No username in session".to_string(),
        ))
    }
}
