use crate::tasks::fetch_artist;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::{Artist, ArtistRelease, CoverArt, Release, Track};
use elder_scrobz_db::scrobble::{Listen, RawScrobble};
use tokio::spawn;
use tracing::{error, info, warn};

mod tasks;

pub async fn populate_scrobbles(pool: &PgPool, uuid: String) -> anyhow::Result<()> {
    let pool = pool.clone();
    info!("Launching scrobble fetch for {}", uuid);
    let Some(scrobble) = RawScrobble::get_by_id(&pool, &uuid).await? else {
        error!("Empty payload for scrobble");
        return anyhow::Ok(());
    };

    let Some(payload) = &scrobble.data.payload.track_metadata else {
        error!("Empty payload for scrobble");
        return anyhow::Ok(());
    };

    let Some(mappings) = payload.mbid_mapping.as_ref() else {
        error!("Empty mappings for scrobble");
        return anyhow::Ok(());
    };

    if let Some((release_mbid, release_name)) = mappings
        .release_mbid
        .as_ref()
        .zip(mappings.recording_name.as_ref())
    {
        if let Some((track_mibd, track_name)) = mappings
            .recording_mbid
            .as_ref()
            .zip(mappings.recording_name.as_ref())
        {
            let artists: Vec<Artist> = mappings
                .artist_mbids
                .as_ref()
                .map(|artit_ids|
                    artit_ids.into_iter()
                        .map(|artist_id| Artist {
                            mbid: artist_id.clone(),
                            name: None,
                            description: None,
                        }))
                .into_iter()
                .flatten()
                .collect();

            for artist in &artists {
                artist.save(&pool).await?;
            }


            if let Some(ca) = mappings.caa_release_mbid.as_ref() {
                CoverArt {
                    mbid: ca.to_string(),
                    url: None,
                }
                .save(&pool)
                .await?;

                Release {
                    mbid: release_mbid.clone(),
                    name: Some(release_name.clone()),
                    cover_art_mbid: ca.to_string(),
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
                }
                .save(&pool)
                .await?;
            }
        };
    };

    Ok(())
}
