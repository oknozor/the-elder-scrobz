use crate::api::listenbrainz::Token;
use crate::error::{AppError, AppResult};
use autometrics::autometrics;
use axum::extract::State;
use axum::{Extension, Json};
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use elder_scrobz_crawler::{get_now_playing, MetadataClient};
use elder_scrobz_db::dlc::CreateErroredScrobble;
use elder_scrobz_db::listens::raw::create::CreateRawScrobble;
use elder_scrobz_db::listens::raw::listenbrainz::{raw, typed, ListenType};
use elder_scrobz_db::user::User;
use elder_scrobz_db::PgPool;
use elder_scrobz_model::events::ScrobzEvent;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::broadcast;
use tracing::warn;

#[derive(Debug, Default, Serialize)]
pub struct Empty {}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/submit-listens",
    summary = "Submit listens",
    params(
        ("Authorization" = String, Header, description = "Token to validate. Format: `Token <token>`")
    ),
    responses(
        (status = 200, description = "Top tracks for user", body = typed::SubmitListens, ),
        (status = 404, description = "User not found", body = AppError)
    ),
    tag = crate::api::SCROBBLES_TAG
)]
#[autometrics]
pub async fn submit_listens(
    State(db): State<PgPool>,
    TypedHeader(auth): TypedHeader<Authorization<Token>>,
    Extension(sse_sender): Extension<broadcast::Sender<ScrobzEvent>>,
    Extension(metadata_client): Extension<MetadataClient>,
    Json(payload): Json<Value>,
) -> AppResult<Json<Empty>> {
    let Some(token) = auth.0.token()? else {
        return Err(AppError::Unauthorized("Missing token".to_string()));
    };

    let Some(user) = User::get_user_id_by_api_key(&db, token).await? else {
        return Err(AppError::Unauthorized("Invalid token".to_string()));
    };

    match serde_json::from_value::<raw::SubmitListens>(payload.clone()) {
        Ok(listens) => match listens.listen_type {
            ListenType::Single | ListenType::Import => {
                CreateRawScrobble::batch_insert(user.username, listens, &db).await?;
            }
            ListenType::PlayingNow => {
                tokio::spawn(async move {
                    for scrobble in listens.payload {
                        let now_playing =
                            get_now_playing(&user.username, &metadata_client, scrobble).await?;
                        sse_sender.send(now_playing)?;
                    }

                    anyhow::Ok(())
                });
            }
        },
        Err(err) => {
            warn!("Failed to deserialize listen, sending to DLC {err}");
            CreateErroredScrobble {
                user_id: user.username,
                data: payload,
            }
            .save(&db)
            .await?;
        }
    };

    Ok(Empty::default().into())
}
