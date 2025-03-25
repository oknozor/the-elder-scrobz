use elder_scrobz_db::PgPool;
use elder_scrobz_db::listens::{Artist, ArtistRelease, Release, Track};
use musicbrainz_rs::client::MusicBrainzClient;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::entity::artist::Artist as MusicBrainzArtist;
use musicbrainz_rs::entity::release::Release as MusicBrainzRelease;
use musicbrainz_rs::{Fetch, FetchCoverart};
use once_cell::sync::Lazy;

static MB_CLIENT: Lazy<MusicBrainzClient> = Lazy::new(|| {
    let mut client = MusicBrainzClient::default();
    client.set_user_agent("elder-scrobz-resolver").unwrap();
    client.max_retries = 5;
    client
});

pub async fn fetch_artist(mbid: String, release_mbid: String, pool: PgPool) -> anyhow::Result<()> {
    let artist = MusicBrainzArtist::fetch()
        .id(&mbid)
        .with_annotations()
        .with_genres()
        .execute_with_client(&MB_CLIENT)
        .await?;

    let artist_id = artist.id;

    let artist = Artist {
        mbid: artist_id.clone(),
        name: Some(artist.name),
        description: artist.annotation,
    };

    artist.save(&pool).await?;

    ArtistRelease {
        artist_mbid: artist_id,
        release_mbid: release_mbid.to_string(),
    }
    .save(&pool)
    .await?;

    Ok(())
}

pub async fn fetch_release(
    release_mbid: String,
    track_mbid: String,
    pool: PgPool,
) -> anyhow::Result<()> {
    let release = MusicBrainzRelease::fetch()
        .id(&release_mbid)
        .with_annotations()
        .with_genres()
        .with_recordings()
        .execute_with_client(&MB_CLIENT)
        .await?;

    let response = release
        .get_coverart()
        .execute_with_client(&MB_CLIENT)
        .await?;

    let coverart_url = match response {
        CoverartResponse::Json(coverart) => Some(coverart.images[0].image.clone()),
        CoverartResponse::Url(url) => Some(url),
    };

    let track = release
        .media
        .into_iter()
        .flatten()
        .map(|media| media.tracks.into_iter().flatten())
        .flatten()
        .find(|track| track.recording.as_ref().map(|r| &r.id) == Some(&track_mbid));

    if let Some(track) = track {
        Track {
            mbid: track_mbid,
            release_mbid: release_mbid.to_string(),
            name: track.title,
            length: track.length.map(|length| length as i32),
            number: Some(track.number),
        }
        .save(&pool)
        .await?;
    };

    let release = Release {
        mbid: release.id,
        name: release.title,
        description: release.annotation,
        cover_art_url: coverart_url,
    };

    release.save(&pool).await?;

    Ok(())
}

#[tokio::test]
async fn test_fetch_artist() {
    let release = MusicBrainzArtist::fetch()
        .id("9fff2f8a-21e6-47de-a2b8-7f449929d43f")
        .with_genres()
        .with_annotations()
        .execute_with_client(&MB_CLIENT)
        .await
        .unwrap();

    println!("{:#?}", release);
}
