use crate::artists::process_artist;
use crate::metadata::MetadataClient;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::releases::Release;
use tracing::{info, warn};

pub async fn try_update_all_releases(
    metadata_client: &MetadataClient,
    pool: &PgPool,
    force: bool,
) -> anyhow::Result<()> {
    let releases = if force {
        Release::all_ids(pool).await?
    } else {
        Release::missing_coverart(pool).await?
    };

    info!("Found {} releases without coverart", releases.len());
    let mut updated = 0;
    for release_id in releases {
        match process_release(&release_id, metadata_client, pool).await {
            Err(err) => {
                warn!("{err}");
                continue;
            }
            Ok(_) => {
                info!("Metadata updated for release {release_id}");
                updated += 1;
            }
        };
    }

    info!("Updated {} releases coverart", updated);

    Ok(())
}

pub async fn process_release(
    release_mbid: &str,
    metadata_client: &MetadataClient,
    db: &PgPool,
) -> anyhow::Result<()> {
    let metadata = metadata_client.get_release_metadata(release_mbid).await?;

    for artist in &metadata.artists_credited {
        process_artist(&artist.mbid, metadata_client, db).await?;
    }

    Release {
        mbid: release_mbid.to_string(),
        name: metadata.name,
        artist_mbid: metadata.artist_mbid,
        description: metadata.description,
        thumbnail_url: metadata.thumbnail_url,
        year: metadata.year,
        subsonic_id: metadata.subsonic_id,
    }
    .save(db)
    .await?;

    Ok(())
}
