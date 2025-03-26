use crate::api::listenbrainz::Token;
use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::{CreateRawScrobble, Listen, SubmitListens};
use elder_scrobz_db::user::User;
use tracing::debug;

#[debug_handler]
#[utoipa::path(
    post,
    path = "/submit-listens",
    params(
        ("Authorization" = String, Header, description = "Token to validate. Format: `Token <token>`")
    ),
    responses(
        (status = 200, description = "Top tracks for user", body = SubmitListens, ),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::SCROBBLES_TAG
)]
pub async fn submit_listens(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Token>>,
    Json(payload): Json<SubmitListens>,
) -> AppResult<()> {
    debug!("{payload:?}");

    let Some(token) = auth.0.token()? else {
        return Err(AppError::Unauthorized("Missing token".to_string()));
    };

    let Some(user) = User::get_user_id_by_api_key(&state.pool, token).await? else {
        return Err(AppError::Unauthorized("Invalid token".to_string()));
    };

    let listens: Vec<Listen> = payload.into();

    let scrobbles = listens
        .into_iter()
        .map(|listen| CreateRawScrobble {
            username: user.username.clone(),
            data: listen.into(),
        })
        .collect();

    CreateRawScrobble::batch_insert(scrobbles, &state.pool).await?;

    Ok(())
}
