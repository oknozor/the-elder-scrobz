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

pub async fn fetch_release(release_mbid: &str, pool: PgPool) -> anyhow::Result<()> {
    let done_already = Release::release_exists_with_cover_art(release_mbid, &pool).await?;
    if done_already {
        return Ok(());
    }

    info!("Fetching release {release_mbid} from MusicBrainz");
    let release = MusicBrainzRelease::fetch()
        .id(&release_mbid)
        .with_annotations()
        .with_genres()
        .with_artist_credits()
        .with_artists()
        .with_recordings()
        .execute_with_client(&MB_CLIENT)
        .await?;

    let artists_credited_on_release: Vec<_> = release
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

    for artist in &artists_credited_on_release {
        artist.save(&pool).await?;
    }

    let response = release
        .get_coverart()
        .execute_with_client(&MB_CLIENT)
        .await?;

    let coverart_url = match response {
        CoverartResponse::Json(coverart) => Some(coverart.images[0].image.clone()),
        CoverartResponse::Url(url) => Some(url),
    };

    if coverart_url.is_none() {
        warn!("No cover art found for release {release_mbid}");
    }

    let release = Release {
        mbid: release.id,
        name: release.title,
        artist_mbid: artists_credited_on_release.first().map(|a| a.mbid.clone()),
        description: release.annotation,
        cover_art_url: coverart_url,
    };

    release.save(&pool).await?;
    info!("Finished fetching release {release_mbid} from MusicBrainz");

    Ok(())
}
