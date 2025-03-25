use crate::tasks::{fetch_artist, fetch_release};
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::raw::RawScrobble;
use elder_scrobz_db::listens::scrobble::Scrobble;
use elder_scrobz_db::listens::{Artist, ArtistRelease, Release, Track};
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
        error!("Empty payload for scrobble");
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
                    artit_ids.into_iter().map(|artist_id| Artist {
                        mbid: artist_id.clone(),
                        name: None,
                        description: None,
                    })
                })
                .into_iter()
                .flatten()
                .collect();

            for artist in &artists {
                spawn(fetch_artist(
                    artist.mbid.clone(),
                    release_mbid.to_string(),
                    pool.clone(),
                ));
                artist.save(&pool).await?;
            }

            Release {
                mbid: release_mbid.clone(),
                name: release_name,
                description: None,
                cover_art_url: None,
            }
            .save(&pool)
            .await?;
            let artist_releases: Vec<ArtistRelease> = artists
                .into_iter()
                .map(|artist| ArtistRelease {
                    artist_mbid: artist.mbid.clone(),
                    release_mbid: release_mbid.clone(),
                })
                .collect();

            for artist_release in artist_releases {
                artist_release.save(&pool).await?;
            }

            Track {
                mbid: track_mibd.clone(),
                release_mbid: release_mbid.clone(),
                name: track_name.clone(),
                number: None,
                length: None,
            }
            .save(&pool)
            .await?;

            spawn(fetch_release(
                release_mbid,
                track_mibd.clone(),
                pool.clone(),
            ));

            Scrobble {
                source_id: scrobble.id.clone(),
                track_id: track_mibd,
                user_id: scrobble.user_id.clone(),
            }
            .save(&pool)
            .await?;
        };
    };

    Ok(())
}
