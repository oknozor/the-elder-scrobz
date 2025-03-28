use crate::error::AppResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum_macros::debug_handler;
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;
use elder_scrobz_resolver::{process, try_update_all_coverart};
use tokio::spawn;
use tracing::info;
use tracing::log::error;
use utoipa::IntoParams;

#[derive(Debug, serde::Deserialize, IntoParams, Default)]
#[serde(default)]
pub struct ScanQuery {
    /// Scan all scrobbles instead of only unprocessed ones
    force: bool,
    /// Only fetch missing coverart
    coverart_only: bool,
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/scan",
    params(ScanQuery),
    responses(
        (status = 200, description = "Launch an asynchronous scrobble scan", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
pub async fn scan_db(
    Query(query): Query<ScanQuery>,
    State(state): State<AppState>,
) -> AppResult<()> {
    if query.coverart_only {
        spawn(async move {
            if let Err(err) = try_update_all_coverart(&state.pool).await {
                error!("Failed to update coverarts: {err}");
            }
        });
        return Ok(());
    }

    let scrobbles = if query.force {
        RawScrobble::all(&state.pool).await?
    } else {
        RawScrobble::get_unprocessed(&state.pool).await?
    };

    info!("Rescanning database for unprocessed scrobbles");
    let pool = state.pool.clone();

    spawn(async move {
        for scrobble in scrobbles {
            let id = scrobble.id.clone();

            info!("Resolving scrobble {id}",);
            match process(scrobble, &pool, query.force).await {
                Ok(id) => info!("Processed scrobble {id}"),
                Err(err) => error!("Failed to process scrobble {id}: {err}"),
            };
        }

        anyhow::Ok(())
    });

    Ok(())
}
