use crate::tasks::fetch_release;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::raw::RawScrobble;
use elder_scrobz_db::listens::scrobble::Scrobble;
use elder_scrobz_db::listens::{Artist, ArtistCredited, Release, Track};
use tokio::spawn;
use tracing::{error, info};

mod tasks;

pub async fn populate_scrobbles(pool: &PgPool, uuid: String) -> anyhow::Result<()> {
    let pool = pool.clone();
    info!("Fetching scrobble metadata for {}", uuid);
    let Some(scrobble) = RawScrobble::get_by_id(&pool, &uuid).await? else {
        error!("Scrobble not found for uuid {uuid}");
        return anyhow::Ok(());
    };

    let data = scrobble.data.0;

    let Some(metadata) = data.payload.track_metadata else {
        error!("Empty track_metadata for scrobble");
        return anyhow::Ok(());
    };

    let Some(release_name) = metadata.release_name else {
        error!("No release name for scrobble");
        return anyhow::Ok(());
    };

    let Some(mappings) = metadata.mbid_mapping else {
        error!("Empty mappings for scrobble");
        return anyhow::Ok(());
    };

    if let Some(release_mbid) = mappings.release_mbid {
        if let Some((track_mibd, track_name)) = mappings.recording_mbid.zip(mappings.recording_name)
        {
            let artists: Vec<Artist> = mappings
                .artist_mbids
                .as_ref()
                .map(|artit_ids| {
                    artit_ids.iter().map(|artist_id| Artist {
                        mbid: artist_id.clone(),
                        name: None,
                        description: None,
                    })
                })
                .into_iter()
                .flatten()
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
                    track_mbid: track_mibd.clone(),
                })
                .collect();

            let Some(main_artist) = artist_credited
                .first()
                .as_ref()
                .map(|a| a.artist_mbid.clone())
            else {
                error!("No main artist for scrobble");
                return anyhow::Ok(());
            };

            let Some(artist_display_name) = metadata.artist_name else {
                error!("No display name for artist {main_artist}");
                return anyhow::Ok(());
            };

            let Some(track_number) = metadata.additional_info.as_ref().map(|i| i.tracknumber)
            else {
                error!("No track_number for track {track_mibd}");
                return anyhow::Ok(());
            };

            let Some(track_length) = metadata.additional_info.map(|i| i.duration_ms) else {
                error!("No track_length track {track_mibd}");
                return anyhow::Ok(());
            };

            Track {
                mbid: track_mibd.clone(),
                artist_mbid: main_artist,
                release_mbid: release_mbid.clone(),
                artist_display_name: Some(artist_display_name),
                name: track_name.clone(),
                number: track_number,
                length: track_length,
            }
            .save(&pool)
            .await?;

            for artist_release in artist_credited {
                artist_release.save(&pool).await?;
            }

            let track_id = track_mibd.clone();
            let pool_copy = pool.clone();
            spawn(async move {
                if let Err(err) = fetch_release(&release_mbid, pool_copy).await {
                    error!("Error fetching release {release_mbid} and track {track_id}: {err}");
                }
            });

            Scrobble {
                source_id: scrobble.id.clone(),
                track_id: track_mibd.to_string(),
                user_id: scrobble.user_id.clone(),
            }
            .save(&pool)
            .await?;
        };
    };

    Ok(())
}
