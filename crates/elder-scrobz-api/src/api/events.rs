use std::{convert::Infallible, time::Duration};

use autometrics::autometrics;
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use axum_extra::{headers::UserAgent, TypedHeader};
use axum_macros::debug_handler;
use futures_util::Stream;
use futures_util::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::state::AppState;

#[debug_handler]
#[utoipa::path(get, path = "/", summary = "Listen to elder scrobz events")]
#[autometrics]
async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("SSE handler connected - {}", user_agent.as_str());
    let mut event_manager = state.event_manager.lock().unwrap();
    let rx = event_manager.sse_sender().subscribe();
    event_manager.handle_connect();
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        match msg {
            Ok(m) => {
                let data = serde_json::to_string(&m).unwrap();
                Some(Ok::<Event, Infallible>(Event::default().data(data)))
            }
            Err(_) => None,
        }
    });

    let keep_alive = KeepAlive::new().interval(Duration::from_secs(1));
    Sse::new(stream).keep_alive(keep_alive)
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(sse_handler))
}
