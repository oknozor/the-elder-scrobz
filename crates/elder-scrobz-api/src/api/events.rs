use std::convert::Infallible;

use autometrics::autometrics;
use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    Extension,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use axum_macros::debug_handler;
use elder_scrobz_db::PgPool;
use elder_scrobz_model::events::ScrobzEvent;
use futures_util::Stream;
use futures_util::StreamExt;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

#[debug_handler]
#[utoipa::path(get, path = "/", summary = "Listen to elder scrobz events")]
#[autometrics]
async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Extension(tx): Extension<broadcast::Sender<ScrobzEvent>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("SSE handler connected - {}", user_agent.as_str());
    let rx = tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(m) => {
                let data = serde_json::to_string(&m).unwrap();
                Some(Ok::<Event, Infallible>(Event::default().data(data)))
            }
            Err(_) => None,
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub fn router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new().routes(routes!(sse_handler))
}
