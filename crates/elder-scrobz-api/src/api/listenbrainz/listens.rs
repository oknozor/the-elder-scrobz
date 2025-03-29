use crate::api::listenbrainz::Token;
use crate::error::{AppError, AppResult};
use axum::{Extension, Json};
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::create::CreateRawScrobble;
use elder_scrobz_db::listens::raw::listenbrainz::{raw, typed, ListenType};
use elder_scrobz_db::user::User;
use elder_scrobz_db::PgPool;
use serde::Serialize;
use tracing::debug;

#[derive(Debug, Default, Serialize)]
pub struct Empty {}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/submit-listens",
    params(
        ("Authorization" = String, Header, description = "Token to validate. Format: `Token <token>`")
    ),
    responses(
        (status = 200, description = "Top tracks for user", body = typed::SubmitListens, ),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::SCROBBLES_TAG
)]
pub async fn submit_listens(
    Extension(db): Extension<PgPool>,
    TypedHeader(auth): TypedHeader<Authorization<Token>>,
    Json(payload): Json<raw::SubmitListens>,
) -> AppResult<Json<Empty>> {
    match payload.listen_type {
        ListenType::Single | ListenType::Import => store_scrobble(&db, auth, payload).await?,
        ListenType::PlayingNow => debug!("Received PlayingNow listen. Ignoring."),
    };

    Ok(Empty::default().into())
}

async fn store_scrobble(
    db: &PgPool,
    auth: Authorization<Token>,
    payload: raw::SubmitListens,
) -> Result<(), AppError> {
    let Some(token) = auth.0.token()? else {
        return Err(AppError::Unauthorized("Missing token".to_string()));
    };

    let Some(user) = User::get_user_id_by_api_key(db, token).await? else {
        return Err(AppError::Unauthorized("Invalid token".to_string()));
    };

    CreateRawScrobble::batch_insert(user.username, payload, db).await?;

    Ok(())
}
