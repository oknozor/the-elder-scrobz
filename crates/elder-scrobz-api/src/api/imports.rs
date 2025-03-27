use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, State};
use axum_extra::extract::JsonLines;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::create::CreateRawScrobble;
use elder_scrobz_db::listens::raw::listenbrainz::{raw, ListenType};
use futures_util::stream::StreamExt;

#[debug_handler]
#[utoipa::path(
    post,
    path = "/users/{id}/import",
    responses(
        (status = 200, description = "Top tracks for user", body = ()),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::SCROBBLES_TAG
)]
pub async fn import_listens(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
    mut stream: JsonLines<serde_json::value::Value>,
) -> AppResult<()> {
    const CHUNK_SIZE: usize = 50;
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);

    while let Some(value) = stream.next().await {
        let value = value?;
        let value = serde_json::from_value(value)?;
        buffer.push(value);

        if buffer.len() == CHUNK_SIZE {
            save_listens(&state, &user_name, buffer).await?;
            buffer = Vec::with_capacity(CHUNK_SIZE);
        }
    }

    if !buffer.is_empty() {
        save_listens(&state, &user_name, buffer).await?;
    }

    Ok(())
}

async fn save_listens(
    state: &AppState,
    username: &str,
    chunk: Vec<raw::SubmitListensPayload>,
) -> AppResult<()> {
    let scrobble = raw::SubmitListens {
        listen_type: ListenType::Import,
        payload: chunk,
    };

    CreateRawScrobble::batch_insert(username.to_string(), scrobble, &state.pool).await?;

    Ok(())
}
