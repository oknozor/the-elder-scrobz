use anyhow::anyhow;
use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::{Artist, Release};
use musicbrainz_rs::client::MusicBrainzClient;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::entity::release::Release as MusicBrainzRelease;
use musicbrainz_rs::{Fetch, FetchCoverart};
use once_cell::sync::Lazy;
use tracing::{info, warn};

static MB_CLIENT: Lazy<MusicBrainzClient> = Lazy::new(|| {
    let mut client = MusicBrainzClient::default();
    client.set_user_agent("elder-scrobz-resolver").unwrap();
    client.max_retries = 5;
    client
});

// TODO: unify with other update process, add discog and lastfm fallback
pub async fn try_update_all_coverart(pool: &PgPool) -> anyhow::Result<()> {
    let releases = Release::missing_coverart(pool).await?;
    info!("Found {} releases without coverart", releases.len());
    let mut updated = 0;
    for release in releases {
        match try_update_coverart(release).await {
            Err(err) => {
                warn!("{err}");
                continue;
            }
            Ok(release) => {
                info!("Updating release coverart {}", release.mbid);
                release.save(pool).await?;
                updated += 1;
            }
        };
    }

    info!("Updated {} releases coverart", updated);

    Ok(())
}

async fn try_update_coverart(release_mbid: String) -> anyhow::Result<Release> {
    let musicbrainz_release = MusicBrainzRelease::fetch()
        .id(&release_mbid)
        .with_annotations()
        .with_genres()
        .with_artist_credits()
        .with_artists()
        .with_recordings()
        .with_release_groups()
        .execute_with_client(&MB_CLIENT)
        .await?;

    let artists_credited_on_release: Vec<_> = musicbrainz_release
        .artist_credit
        .iter()
        .flatten()
        .map(|artist_credit| &artist_credit.artist)
        .map(|artist| Artist {
            mbid: artist.id.clone(),
            name: Some(artist.name.clone()),
            description: artist.annotation.clone(),
        })
        .collect();

    let cover_art_url = musicbrainz_release
        .get_coverart()
        .front()
        .execute_with_client(&MB_CLIENT)
        .await;

    let cover_art_url = match cover_art_url {
        Err(e) => {
            warn!(
                "Failed to get coverart for release {release_mbid}: {e}. Falling back to release group",
            );

            match musicbrainz_release.release_group {
                None => {
                    return Err(anyhow!("No release group, skipping release coverart"));
                }
                Some(group) => {
                    group
                        .get_coverart()
                        .front()
                        .execute_with_client(&MB_CLIENT)
                        .await
                }
            }
        }
        Ok(cover_art_url) => Ok(cover_art_url),
    };

    let cover_art_url = match cover_art_url {
        Ok(cover_art_url) => cover_art_url,
        Err(e) => return Err(anyhow!("failed to get coverart for release group, {e}")),
    };

    let cover_art_url = match cover_art_url {
        CoverartResponse::Json(coverart) => coverart.images[0].image.clone(),
        CoverartResponse::Url(url) => url,
    };

    Ok(Release {
        mbid: release_mbid,
        name: musicbrainz_release.title,
        artist_mbid: artists_credited_on_release.first().map(|a| a.mbid.clone()),
        description: musicbrainz_release.annotation,
        cover_art_url: Some(cover_art_url),
    })
}

pub async fn fetch_release(release_mbid: &str, pool: PgPool, force: bool) -> anyhow::Result<()> {
    let done_already = Release::release_exists_with_cover_art(release_mbid, &pool).await?;
    if done_already && !force {
        return Ok(());
    }

    let release = try_update_coverart(release_mbid.to_string()).await?;
    release.save(&pool).await?;
    info!("Finished fetching release {release_mbid} from MusicBrainz");

    Ok(())
}
