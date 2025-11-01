use axum::extract::State;
use axum::routing::get;
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};
use axum::{Extension, Router};
use axum_macros::debug_handler;
use axum_session_auth::AuthSession;
use axum_session_sqlx::SessionPgPool;
use elder_scrobz_db::user::{CreateUser, User};
use elder_scrobz_db::PgPool;
use oauth2::{CsrfToken, Scope};
use serde::Deserialize;

use crate::error::AppResult;
use crate::oauth::client::OauthClient;
use crate::oauth::user;
use crate::state::AppState;

pub type Session = AuthSession<user::AuthenticatedUser, String, SessionPgPool, PgPool>;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(openid_auth))
        .route("/authorized", get(login_authorized))
        .route("/logout", get(logout))
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

#[debug_handler]
pub async fn openid_auth(Extension(client): Extension<OauthClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

#[debug_handler]
pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    session: Session,
    Extension(client): Extension<OauthClient>,
    State(state): State<AppState>,
) -> AppResult<Redirect> {
    let authenticated_username = client.get_token(query.code).await?;

    if User::get_by_username(&state.db, &authenticated_username)
        .await?
        .is_none()
    {
        let is_first_user = User::count(&state.db).await? == 0;

        CreateUser {
            username: authenticated_username.clone(),
            admin: is_first_user,
        }
        .insert(&state.db)
        .await?;
    }

    session.login_user(authenticated_username);
    Ok(Redirect::to("/"))
}

#[debug_handler]
pub async fn logout(session: Session) -> AppResult<Redirect> {
    session.logout_user();
    Ok(Redirect::to("/"))
}
