use crate::error::AppResult;
use autometrics::autometrics;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use elder_scrobz_crawler::{process_scrobble, try_update_all_coverart, MetadataClient};
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;
use elder_scrobz_db::stats::Stats;
use elder_scrobz_db::PgPool;
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
    summary = "Scan database",
    description = "Scan database and fetch medatada",
    params(ScanQuery),
    responses(
        (status = 200, description = "Launch an asynchronous scrobble scan", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn scan_db(
    Query(query): Query<ScanQuery>,
    Extension(db): Extension<PgPool>,
    Extension(client): Extension<MetadataClient>,
) -> AppResult<()> {
    if query.coverart_only {
        spawn(async move {
            if let Err(err) = try_update_all_coverart(&client, &db).await {
                error!("Failed to update coverarts: {err}");
            }
        });
        return Ok(());
    }

    let scrobbles = if query.force {
        RawScrobble::all(&db).await?
    } else {
        RawScrobble::get_unprocessed(&db).await?
    };

    info!("Rescanning database for unprocessed scrobbles");
    let pool = db.clone();

    spawn(async move {
        for scrobble in scrobbles {
            let id = scrobble.id.clone();

            info!("Resolving scrobble {id}",);
            match process_scrobble(scrobble, &pool).await {
                Ok(id) => info!("Processed scrobble {id}"),
                Err(err) => error!("Failed to process scrobble {id}: {err}"),
            };
        }

        anyhow::Ok(())
    });

    Ok(())
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/stats",
    summary = "Database statistics",
    description = "Statistics on database metadata",
    responses(
        (status = 200, body = Stats, description = "Return detailed stats about the scrobble database", content_type = "application/json"),
    ),
    tag = crate::api::ADMIN_TAG
)]
#[autometrics]
pub async fn stats(Extension(db): Extension<PgPool>) -> AppResult<Json<Stats>> {
    Ok(Json(Stats::get(&db).await?))
}
