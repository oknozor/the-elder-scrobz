use crate::tasks::fetch_release;
use anyhow::anyhow;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::raw::RawScrobble;
use elder_scrobz_db::listens::scrobble::Scrobble;
use elder_scrobz_db::listens::{Artist, ArtistCredited, Release, Track};
use sqlx::postgres::PgListener;
use tokio::spawn;
use tracing::{error, info};

mod tasks;

pub async fn pg_listener(pool: PgPool) -> anyhow::Result<()> {
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("new_insert").await?;
    let mut retry_count = 3;

    while retry_count > 0 {
        while let Ok(notification) = listener.recv().await {
            let scrobble = serde_json::from_str::<RawScrobble>(notification.payload())?;
            match process(scrobble, &pool).await {
                Ok(id) => {
                    info!("Processed scrobble {id}");
                }
                Err(err) => {
                    error!("Error processing scrobble: {err}");
                }
            };
        }

        retry_count -= 1;
    }

    error!("PgListener on scrobbles_raw_insert_trigger exited");
    Ok(())
}

pub async fn process(scrobble: RawScrobble, pool: &PgPool) -> anyhow::Result<String> {
    let pool = pool.clone();
    let scrobble_id = scrobble.id;
    let user_id = scrobble.user_id;
    let additional_info = scrobble
        .data
        .0
        .payload
        .track_metadata
        .additional_info
        .as_ref();

    let mappings = scrobble.data.0.payload.track_metadata.mbid_mapping.as_ref();

    let track_number = additional_info.and_then(|info| info.tracknumber);

    let track_duration = additional_info.and_then(|info| info.duration_ms);

    let track_name = scrobble.data.0.payload.track_metadata.track_name;
    let artist_name = scrobble.data.0.payload.track_metadata.artist_name;
    let release_name = scrobble.data.0.payload.track_metadata.release_name;

    let recording_mbid = additional_info
        .and_then(|info| info.recording_mbid.clone())
        .or_else(|| mappings.and_then(|mapping| mapping.recording_mbid.clone()))
        .ok_or(anyhow::anyhow!(
            "No recording_mbid found for scrobble {scrobble_id}"
        ))?;

    let release_mbid = additional_info
        .and_then(|info| info.release_mbid.clone())
        .or_else(|| mappings.and_then(|mapping| mapping.release_mbid.clone()))
        .ok_or(anyhow::anyhow!(
            "No release_mbid found for scrobble {scrobble_id}"
        ))?;

    let artist_mbids = additional_info
        .and_then(|info| info.artist_mbids.clone())
        .or_else(|| mappings.and_then(|mapping| mapping.artist_mbids.clone()))
        .ok_or(anyhow::anyhow!(
            "No artist_mbids found for scrobble {scrobble_id}"
        ))?;

    info!("Fetching scrobble({scrobble_id}) metadata for {artist_name} - {track_name}",);
    let artists: Vec<Artist> = artist_mbids
        .into_iter()
        .map(|mbid| Artist {
            mbid,
            name: None,
            description: None,
        })
        .collect();

    for artist in &artists {
        artist.save(&pool).await?;
    }

    Release {
        mbid: release_mbid.clone(),
        name: release_name,
        artist_mbid: None,
        description: None,
        cover_art_url: None,
    }
    .save(&pool)
    .await?;

    let artist_credited: Vec<ArtistCredited> = artists
        .into_iter()
        .map(|artist| ArtistCredited {
            artist_mbid: artist.mbid.clone(),
            track_mbid: recording_mbid.clone(),
        })
        .collect();

    let Some(main_artist) = artist_credited
        .first()
        .as_ref()
        .map(|a| a.artist_mbid.clone())
    else {
        return Err(anyhow!("No main artist for scrobble {scrobble_id}"));
    };

    Track {
        mbid: recording_mbid.clone(),
        artist_mbid: main_artist,
        release_mbid: release_mbid.clone(),
        artist_display_name: Some(artist_name),
        name: track_name.clone(),
        number: track_number,
        length: track_duration,
    }
    .save(&pool)
    .await?;

    for artist_release in artist_credited {
        artist_release.save(&pool).await?;
    }

    let pool_copy = pool.clone();
    spawn(async move {
        if let Err(err) = fetch_release(&release_mbid, pool_copy).await {
            error!("Error fetching release {release_mbid}: {err}");
        }
    });

    Scrobble {
        source_id: scrobble_id.clone(),
        track_id: recording_mbid,
        user_id,
    }
    .save(&pool)
    .await?;

    Ok(scrobble_id)
}
