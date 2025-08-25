use crate::MetadataClient;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::artists::Artist;
use tracing::{info, warn};

pub async fn process_artist(
    mbid: &str,
    metadata_client: &MetadataClient,
    db: &PgPool,
) -> anyhow::Result<()> {
    let metadata = metadata_client.get_artist_metadata(mbid).await?;

    Artist {
        mbid: mbid.to_string(),
        name: metadata.name,
        description: metadata.description,
        thumbnail_url: metadata.thumbnail_url,
        subsonic_id: metadata.subsonic_id,
    }
    .save(db)
    .await?;

    Ok(())
}

pub async fn try_update_all_artists(
    metadata_client: &MetadataClient,
    pool: &PgPool,
    force: bool,
) -> anyhow::Result<()> {
    let artists = if force {
        Artist::all_ids(pool).await?
    } else {
        Artist::with_missing_metadata(pool).await?
    };

    info!("Found {} artists without metadata", artists.len());
    let mut updated = 0;
    for artist_id in artists {
        match process_artist(&artist_id, metadata_client, pool).await {
            Err(err) => {
                warn!("{err}");
                continue;
            }
            Ok(_) => {
                info!("Metadata updated for artist {artist_id}");
                updated += 1;
            }
        };
    }

    info!("Updated {} artists metadata", updated);

    Ok(())
}
