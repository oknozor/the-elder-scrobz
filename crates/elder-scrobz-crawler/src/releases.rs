use crate::metadata::MetadataClient;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::{Artist, Release};
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
