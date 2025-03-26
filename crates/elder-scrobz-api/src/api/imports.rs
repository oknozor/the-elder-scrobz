use crate::error::{AppError, AppResult};
use crate::AppState;
use axum::extract::{Path, State};
use axum_extra::extract::JsonLines;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::{CreateRawScrobble, Listen, ListenType, SubmitListensPayload};
use elder_scrobz_resolver::populate_scrobbles;
use futures_util::stream::StreamExt;
use tracing::error;

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
        .map(|payload| CreateRawScrobble {
            user_id: user_id.to_string(),
            data: Listen {
                listen_type: ListenType::Import,
                payload,
            }
            .into(),
        })
        .collect();

    let uuids = CreateRawScrobble::batch_insert(chunk, &state.pool).await?;

    for scrobble_id in uuids {
        let pool = state.pool.clone();
        tokio::spawn(async move {
            if let Err(err) = populate_scrobbles(&pool, scrobble_id).await {
                error!("{err}");
            }
        });
    }

    Ok(())
}
