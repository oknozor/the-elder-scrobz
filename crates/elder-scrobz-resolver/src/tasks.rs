use elder_scrobz_db::listens::{Artist, ArtistRelease};
use elder_scrobz_db::PgPool;
use musicbrainz_rs::entity::artist::Artist as MusicBrainzArtist;
use musicbrainz_rs::Fetch;

pub async fn fetch_artist(mbid: &str, release_mbid: &str, pool: PgPool) -> anyhow::Result<()> {
    let artist = MusicBrainzArtist::fetch().id(mbid).execute().await?;
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
    }.save(&pool).await?;

    Ok(())
}