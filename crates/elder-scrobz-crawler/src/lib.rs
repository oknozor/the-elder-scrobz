use std::path::PathBuf;

use crate::releases::process_release;
use anyhow::Result;
use anyhow::anyhow;
use artists::process_artist;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::cover_art;
use elder_scrobz_db::listens::artists::ArtistCredited;
use elder_scrobz_db::listens::raw::listenbrainz::raw::SubmitListensPayload;
use elder_scrobz_db::listens::raw::listenbrainz::typed;
use elder_scrobz_db::listens::raw::scrobble::{RawScrobble, TypedScrobble};
use elder_scrobz_db::listens::scrobble::Scrobble;
use elder_scrobz_model::events::ScrobzEvent;
use sqlx::postgres::{PgListener, PgNotification};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use tracing::{error, info, warn};

mod artists;
mod coverart;
mod metadata;
mod releases;

pub use artists::try_update_all_artists;
use elder_scrobz_db::listens::artists::Artist;
use elder_scrobz_db::listens::releases::Release;
use elder_scrobz_db::listens::tracks::Track;
pub use metadata::MetadataClient;
pub use releases::try_update_all_releases;

pub struct ScrobbleCrawler {
    pool: PgPool,
    pg_listener: PgListener,
    client: reqwest::Client,
    coverart_path: PathBuf,
    metadata_client: MetadataClient,
    cancellation_token: CancellationToken,
}

pub struct DiscogsConfig {
    pub key: String,
    pub secret: String,
}

pub struct NavidromeConfig {
    pub username: String,
    pub password: String,
    pub server_url: String,
}

impl ScrobbleCrawler {
    pub async fn create(
        pool: PgPool,
        coverart_path: PathBuf,
        discogs_config: DiscogsConfig,
        navidrom_config: NavidromeConfig,
        cancellation_token: CancellationToken,
    ) -> Result<Self> {
        let pg_listener = PgListener::connect_with(&pool).await?;
        Ok(Self {
            pool,
            pg_listener,
            client: reqwest::Client::new(),
            coverart_path,
            metadata_client: MetadataClient::new(
                discogs_config.key,
                discogs_config.secret,
                navidrom_config.username,
                navidrom_config.password,
                navidrom_config.server_url,
            ),
            cancellation_token,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let token = self.cancellation_token.clone();
        select! {
            _ = token.cancelled() => Ok(()),
            result = self.listen() => result,
        }
    }

    async fn listen(&mut self) -> Result<()> {
        info!("Starting PgListener on scrobbles_raw_insert_trigger");
        self.pg_listener
            .listen_all([
                "new_insert",
                "coverart_updated",
                "thumbnail_updated",
                "release_inserted",
                "artist_inserted",
            ])
            .await?;

        let mut retry_count = 3;

        while retry_count > 0 {
            while let Ok(notification) = self.pg_listener.recv().await {
                match notification.channel() {
                    "new_insert" => {
                        if self.handle_scrobble_insert(notification).await {
                            continue;
                        }
                    }
                    "release_inserted" => {
                        if self.handle_release_insert(notification).await {
                            continue;
                        }
                    }
                    "artist_inserted" => {
                        if self.handle_artist_insert(notification).await {
                            continue;
                        }
                    }
                    "coverart_updated" => {
                        let release: serde_json::Value =
                            serde_json::from_str(notification.payload())?;
                        let mbid = release["mbid"].as_str().expect("mbid is not a string");
                        if let Some(coverart) = release["cover_art_url"].as_str() {
                            info!("Downloading coverart for release {}", mbid);
                            if let Err(err) = self.download_image(coverart, mbid).await {
                                error!("Failed to download cover art: {}", err);
                            }
                        }
                    }
                    "thumbnail_updated" => {
                        let artist: Artist = serde_json::from_str(notification.payload())?;
                        info!("Downloading thumbnail for artist {}", artist.mbid);
                        if let Some(thumbnail) = artist.thumbnail_url
                            && let Err(err) = self.download_image(&thumbnail, &artist.mbid).await
                        {
                            error!("Failed to download cover art: {}", err);
                        }
                    }
                    _ => {}
                }
            }

            warn!("pg listener failed, trying to reconnect");
            retry_count -= 1;
        }

        error!("PgListener exited");
        Ok(())
    }

    async fn handle_scrobble_insert(&mut self, notification: PgNotification) -> bool {
        info!("Processing new scrobble");
        let Ok(scrobble) = serde_json::from_str::<RawScrobble>(notification.payload()) else {
            let payload = notification.payload();
            error!("Failed to parse scrobble: {payload}");
            return true;
        };

        match process_scrobble(scrobble, &self.metadata_client, &self.pool).await {
            Ok(id) => {
                info!("Processed scrobble {id}");
            }
            Err(err) => {
                error!("Error processing scrobble: {err}");
            }
        };
        false
    }

    async fn handle_release_insert(&mut self, notification: PgNotification) -> bool {
        info!("Processing new release");
        let Ok(release) = serde_json::from_str::<Release>(notification.payload()) else {
            let payload = notification.payload();
            error!("Failed to parse release: {payload}");
            return true;
        };

        match process_release(&release.mbid, &self.metadata_client, &self.pool).await {
            Ok(_) => {
                info!("Processed release {}", &release.mbid);
            }
            Err(err) => {
                error!("Error processing release: {err}");
            }
        };
        false
    }

    async fn handle_artist_insert(&mut self, notification: PgNotification) -> bool {
        info!("Processing new artist");
        let Ok(artist) = serde_json::from_str::<Artist>(notification.payload()) else {
            let payload = notification.payload();
            error!("Failed to parse artist: {payload}");
            return true;
        };

        match process_artist(&artist.mbid, &self.metadata_client, &self.pool).await {
            Ok(_) => {
                info!("Processed artist {}", artist.mbid);
            }
            Err(err) => {
                error!("Error processing artist: {err}");
            }
        };
        false
    }
}

pub async fn process_scrobble(
    scrobble: RawScrobble,
    client: &MetadataClient,
    pool: &PgPool,
) -> Result<String> {
    let scrobble: TypedScrobble = scrobble.try_into()?;
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
            subsonic_id: None,
            name: None,
            description: None,
            thumbnail_url: None,
        })
        .collect();

    for artist in &artists {
        artist.save(&pool).await?;
    }

    Release {
        mbid: release_mbid.to_string(),
        name: release_name.to_string(),
        artist_mbid: None,
        description: None,
        thumbnail_url: None,
        year: None,
        subsonic_id: None,
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

    let subsonic_track = client
        .subsonic_client
        .search_by_mbid(&recording_mbid)
        .await?;

    let subsonic_id = subsonic_track
        .song
        .first()
        .map(|subsonic_track| subsonic_track.id.clone());

    Track {
        mbid: recording_mbid.clone(),
        artist_mbid: main_artist,
        release_mbid: release_mbid.to_string(),
        artist_display_name: Some(artist_name.to_string()),
        name: track_name.to_string(),
        number: track_number,
        length: track_duration,
        subsonic_id,
    }
    .save(&pool)
    .await?;

    for artist_release in artist_credited {
        artist_release.save(&pool).await?;
    }

    Scrobble {
        source_id: scrobble_id.clone(),
        track_id: recording_mbid,
        user_id,
    }
    .save(&pool)
    .await?;

    TypedScrobble::set_processed(&pool, &scrobble_id).await?;
    Ok(scrobble_id)
}

pub async fn get_now_playing(
    user: &str,
    client: &MetadataClient,
    db: &PgPool,
    scrobble: SubmitListensPayload,
) -> Result<ScrobzEvent> {
    let user = user.to_string();
    let scrobble: typed::SubmitListensPayload = scrobble.try_into()?;
    let track_name = scrobble.track_name().to_string();
    let artist = scrobble.artist_name().to_string();
    let album = scrobble.release_name().to_string();
    let track_duration = scrobble.track_duration().unwrap_or(0);

    debug!("Fetching cover art for now playing : {track_name} - {artist} ({user})",);
    let cover_art_url = match scrobble.release_mbid() {
        Some(mbid) => match cover_art::get(mbid, db).await? {
            Some(ca) => Some(ca),
            None => client.get_release_cover_art(mbid).await?,
        },
        None => None,
    };

    Ok(ScrobzEvent::PlayingNow {
        user,
        track_name,
        artist,
        album,
        cover_art_url,
        track_duration,
    })
}
