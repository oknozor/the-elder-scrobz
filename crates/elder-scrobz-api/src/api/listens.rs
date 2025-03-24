use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::header::ToStrError;
use axum::http::HeaderValue;
use axum::Json;
use axum_extra::extract::JsonLines;
use axum_extra::headers::authorization::Credentials;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use elder_scrobz_db::scrobble::{
    CreateScrobble, Listen, ListenType, SubmitListens, SubmitListensPayload,
};
use elder_scrobz_db::user::User;
use futures_util::stream::StreamExt;

#[debug_handler]
#[utoipa::path(
    post,
    path = "/1/submit-listens",
    responses(
        (status = 200, description = "Top tracks for user", body = ()),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn submit_listens(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Token>>,
    Json(payload): Json<SubmitListens>,
) -> AppResult<()> {
    let Some(token) = auth.0.token()? else {
        return Err(AppError::Unauthorized("Missing token".to_string()));
    };

    let Some(user_id) = User::get_user_id_by_api_key(&state.pool, token).await? else {
        return Err(AppError::Unauthorized("Invalid token".to_string()));
    };

    let listens: Vec<Listen> = payload.into();
    let scrobbles = listens
        .into_iter()
        .map(|listen| CreateScrobble {
            user_id: user_id.to_string(),
            data: listen.into(),
        })
        .collect();

    CreateScrobble::batch_insert(scrobbles, &state.pool).await?;
    Ok(())
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/users/{id}/import",
    responses(
        (status = 200, description = "Top tracks for user", body = ()),
        (status = 404, description = "User not found", body = AppError)
    )
)]
pub async fn import_listens(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    mut stream: JsonLines<serde_json::value::Value>,
) -> AppResult<()> {
    const CHUNK_SIZE: usize = 50;
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);

    while let Some(value) = stream.next().await {
        let value = value?;
        let value = serde_json::from_value(value)?;
        buffer.push(value);

        if buffer.len() == CHUNK_SIZE {
            save_listens(&state, &user_id, buffer).await?;
            buffer = Vec::with_capacity(CHUNK_SIZE);
        }
    }

    if !buffer.is_empty() {
        save_listens(&state, &user_id, buffer).await?;
    }

    Ok(())
}

async fn save_listens(
    state: &AppState,
    user_id: &str,
    chunk: Vec<SubmitListensPayload>,
) -> AppResult<()> {
    let chunk = chunk
        .into_iter()
        .map(|payload| CreateScrobble {
            user_id: user_id.to_string(),
            data: Listen {
                listen_type: ListenType::Import,
                payload,
            }
            .into(),
        })
        .collect();

    CreateScrobble::batch_insert(chunk, &state.pool).await?;
    Ok(())
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
