use crate::metadata::MetadataClient;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::{Artist, Release};
use tracing::{info, warn};

// TODO: unify with other update process, add discog and lastfm fallback
pub async fn try_update_all_coverart(
    metadata_client: &MetadataClient,
    pool: &PgPool,
) -> anyhow::Result<()> {
    let releases = Release::missing_coverart(pool).await?;
    info!("Found {} releases without coverart", releases.len());
    let mut updated = 0;
    for release in releases {
        match process_release(&release, metadata_client, pool).await {
            Err(err) => {
                warn!("{err}");
                continue;
            }
            Ok(_) => {
                info!("Metadata updated for release {}", release);
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

    Release {
        mbid: release_mbid.to_string(),
        name: metadata.name,
        artist_mbid: metadata.artist_mbid,
        description: metadata.description,
        cover_art_url: metadata.cover_art_url,
    }
    .save(db)
    .await?;

    Ok(())
}

pub async fn process_artist(
    release_mbid: &str,
    metadata_client: &MetadataClient,
    db: &PgPool,
) -> anyhow::Result<()> {
    let metadata = metadata_client.get_artist_metadata(release_mbid).await?;

    Artist {
        mbid: release_mbid.to_string(),
        name: metadata.name,
        description: metadata.description,
        thumbnail_url: metadata.thumbnail_url,
    }
    .save(db)
    .await?;

    Ok(())
}
