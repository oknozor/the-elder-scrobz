use crate::tasks::fetch_release;
use anyhow::Result;
use anyhow::anyhow;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::raw::RawScrobble;
use elder_scrobz_db::listens::scrobble::Scrobble;
use elder_scrobz_db::listens::{Artist, ArtistCredited, Release, Track};
use sqlx::postgres::PgListener;
use tracing::{error, info, warn};

mod tasks;

pub struct ScrobbleResolver {
    pool: PgPool,
    pg_listener: PgListener,
}

impl ScrobbleResolver {
    pub async fn create(pool: PgPool) -> Result<Self> {
        let pg_listener = PgListener::connect_with(&pool).await?;
        Ok(Self { pool, pg_listener })
    }

    pub async fn listen(&mut self) -> anyhow::Result<()> {
        info!("Starting PgListener on scrobbles_raw_insert_trigger");
        self.pg_listener.listen("new_insert").await?;

        let mut retry_count = 3;

        while retry_count > 0 {
            while let Ok(notification) = self.pg_listener.recv().await {
                info!("Processing new_insert");
                let Ok(scrobble) = serde_json::from_str::<RawScrobble>(notification.payload())
                else {
                    let payload = notification.payload();
                    error!("Failed to parse scrobble: {payload}");
                    continue;
                };

                match process(scrobble, &self.pool).await {
                    Ok(id) => {
                        info!("Processed scrobble {id}");
                    }
                    Err(err) => {
                        error!("Error processing scrobble: {err}");
                    }
                };
            }

            warn!("pg listener failed, trying to reconnect");
            retry_count -= 1;
        }

        error!("PgListener on scrobbles_raw_insert_trigger exited");
        Ok(())
    }
}

// Todo: wrap everything in a transaction
pub async fn process(scrobble: RawScrobble, pool: &PgPool) -> anyhow::Result<String> {
    let pool = pool.clone();
    let scrobble_id = scrobble.id();
    let user_id = scrobble.user_id();
    let track_number = scrobble.track_number();
    let track_duration = scrobble.track_duration();
    let track_name = scrobble.track_name();
    let artist_name = scrobble.artist_name();
    let release_name = scrobble.release_name();

    let recording_mbid = scrobble.recording_mbid().ok_or(anyhow::anyhow!(
        "No recording_mbid found for scrobble {scrobble_id}"
    ))?;

    let release_mbid = scrobble.release_mbid().ok_or(anyhow::anyhow!(
        "No release_mbid found for scrobble {scrobble_id}"
    ))?;

    let artist_mbids = scrobble.artist_mbids().ok_or(anyhow::anyhow!(
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

    fetch_release(&release_mbid, pool_copy).await?;

    Scrobble {
        source_id: scrobble_id.clone(),
        track_id: recording_mbid,
        user_id,
    }
    .save(&pool)
    .await?;

    RawScrobble::set_processed(&pool, &scrobble_id).await?;
    Ok(scrobble_id)
}
