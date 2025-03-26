use crate::error::AppResult;
use crate::AppState;
use axum::extract::State;
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::RawScrobble;
use elder_scrobz_resolver::process;
use tokio::spawn;
use tracing::info;

#[debug_handler]
#[utoipa::path(
    post,
    path = "/scan",
    responses(
        (status = 200, description = "Launch an asynchronous scrobble scan", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
pub async fn scan_db(State(state): State<AppState>) -> AppResult<()> {
    let scrobbles = RawScrobble::get_unprocessed(&state.pool).await?;
    info!("Rescanning database for unprocessed scrobbles");

    let pool = state.pool.clone();

    spawn(async move {
        for scrobble in scrobbles {
            let id = process(scrobble, &pool).await?;
            info!("Processed scrobble {id}");
        }
        anyhow::Ok(())
    });

    Ok(())
}
