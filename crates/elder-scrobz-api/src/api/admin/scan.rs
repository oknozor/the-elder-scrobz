use crate::error::AppResult;
use autometrics::autometrics;
use axum::extract::{Query, State};
use axum::Extension;
use axum_macros::debug_handler;
use elder_scrobz_crawler::{
    process_scrobble, try_update_all_artists, try_update_all_releases, MetadataClient,
};
use elder_scrobz_db::listens::raw::scrobble::RawScrobble;
use elder_scrobz_db::PgPool;
use tokio::spawn;
use tracing::info;
use tracing::log::error;
use utoipa::IntoParams;

#[derive(Debug, serde::Deserialize, IntoParams)]
#[serde(default)]
pub struct ScanQuery {
    /// Scan all scrobbles instead of only unprocessed ones
    force: bool,
    /// Only fetch missing coverart
    artists: bool,
    releases: bool,
    scrobbles: bool,
}

impl Default for ScanQuery {
    fn default() -> Self {
        Self {
            force: false,
            artists: false,
            releases: false,
            scrobbles: true,
        }
    }
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
    State(db): State<PgPool>,
    Extension(client): Extension<MetadataClient>,
) -> AppResult<()> {
    if query.releases {
        let db = db.clone();
        let client = client.clone();
        spawn(async move {
            if let Err(err) = try_update_all_releases(&client, &db, query.force).await {
                error!("Failed to update releases: {err}");
            }
        });
    }

    if query.artists {
        let db = db.clone();
        let client = client.clone();
        spawn(async move {
            if let Err(err) = try_update_all_artists(&client, &db, query.force).await {
                error!("Failed to update artists: {err}");
            }
        });
    }

    if query.scrobbles {
        let scrobbles = if query.force {
            RawScrobble::all(&db).await?
        } else {
            RawScrobble::get_unprocessed(&db).await?
        };

        info!("Rescanning database for unprocessed scrobbles");

        spawn(async move {
            for scrobble in scrobbles {
                let id = scrobble.id.clone();

                info!("Resolving scrobble {id}",);
                match process_scrobble(scrobble, &client, &db).await {
                    Ok(id) => info!("Processed scrobble {id}"),
                    Err(err) => error!("Failed to process scrobble {id}: {err}"),
                };
            }

            anyhow::Ok(())
        });
    }

    Ok(())
}
