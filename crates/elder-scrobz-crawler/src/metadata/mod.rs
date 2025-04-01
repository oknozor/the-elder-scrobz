use anyhow::anyhow;
use musicbrainz_rs::client::MusicBrainzClient;
use musicbrainz_rs::entity::CoverartResponse;
use musicbrainz_rs::entity::artist::Artist;
use musicbrainz_rs::entity::relations::{Relation, RelationContent};
use musicbrainz_rs::entity::release::Release;
use musicbrainz_rs::{Fetch, FetchCoverart};
use once_cell::sync::Lazy;
use tracing::warn;

mod discogs;
mod wiki;

static MB_CLIENT: Lazy<MusicBrainzClient> = Lazy::new(|| {
    let mut client = MusicBrainzClient::default();
    client.set_user_agent("elder-scrobz-crawler").unwrap();
    client.max_retries = 5;
    client
});

#[derive(Debug, Clone)]
pub struct MetadataClient {
    client: reqwest::Client,
    discogs_token: String,
}

impl MetadataClient {
    pub fn new(discogs_token: String) -> Self {
        Self {
            client: Default::default(),
            discogs_token,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArtistMetadata {
    pub mbid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
}

impl ArtistMetadata {
    fn new(mbid: &str) -> ArtistMetadata {
        ArtistMetadata {
            mbid: mbid.to_string(),
            name: None,
            description: None,
            thumbnail_url: None,
        }
    }
}

#[derive(Debug)]
pub struct ReleaseMetadata {
    pub mbid: String,
    pub name: String,
    pub artist_mbid: Option<String>,
    pub description: Option<String>,
    pub cover_art_url: Option<String>,
    pub artists_credited: Vec<ArtistMetadata>,
}

impl MetadataClient {
    pub async fn get_artist_metadata(&self, artist_mbid: &str) -> anyhow::Result<ArtistMetadata> {
        let artist = Artist::fetch()
            .id(artist_mbid)
            .with_url_relations()
            .execute_with_client(&MB_CLIENT)
            .await?;

        let relations = artist.relations.unwrap_or_default();

        let wikidata_id: Option<String> = extract_relation_id(&relations, "wikidata");
        let discogs_id: Option<String> = extract_relation_id(&relations, "discogs");
        let mut artist_metadata = ArtistMetadata::new(artist_mbid);

        artist_metadata.name = Some(artist.name);
        if let Some(discogs_id) = discogs_id {
            let artist = self.get_discogs_artist(&discogs_id).await?;
            artist_metadata.thumbnail_url = artist_metadata.thumbnail_url.or_else(|| {
                artist
                    .images
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|image| image.r#type == "primary")
                    .filter(|image| !image.resource_url.is_empty())
                    .map(|image| image.resource_url)
                    .next()
            });

            artist_metadata.description = Some(artist.profile);
        }

        if let Some(wikidata_id) = wikidata_id {
            let item = self.get_wikidata(&wikidata_id).await?;
            let wiki_title = item
                .entities
                .get(&wikidata_id)
                .and_then(|e| e.sitelinks.get("enwiki"))
                .map(|d| d.title.clone());

            if let Some(title) = wiki_title {
                let description = self.get_wikipedia_description(&title).await?;
                artist_metadata.description = Some(description);
            }
        }

        Ok(artist_metadata)
    }

    pub async fn get_release_metadata(
        &self,
        release_mbid: &str,
    ) -> anyhow::Result<ReleaseMetadata> {
        let release = Release::fetch()
            .id(release_mbid)
            .with_annotations()
            .with_genres()
            .with_artist_credits()
            .with_artists()
            .with_recordings()
            .with_release_groups()
            .with_url_relations()
            .execute_with_client(&MB_CLIENT)
            .await?;

        let artists_credited: Vec<_> = release
            .artist_credit
            .iter()
            .flatten()
            .map(|artist_credit| &artist_credit.artist)
            .map(|artist| ArtistMetadata {
                mbid: artist.id.clone(),
                name: Some(artist.name.clone()),
                description: None,
                thumbnail_url: None,
            })
            .collect();

        let cover_art_url = release
            .get_coverart()
            .front()
            .res_250()
            .execute_with_client(&MB_CLIENT)
            .await;

        let cover_art_url = match cover_art_url {
            Err(e) => {
                warn!(
                    "Failed to get coverart for release {release_mbid}: {e}. Falling back to release group",
                );

                match release.release_group {
                    None => Err(anyhow!("No release group, skipping release coverart")),
                    Some(group) => Ok(group
                        .get_coverart()
                        .front()
                        .res_250()
                        .execute_with_client(&MB_CLIENT)
                        .await?),
                }
            }
            Ok(cover_art_url) => Ok(cover_art_url),
        };

        let cover_art_url = cover_art_url
            .ok()
            .and_then(|coverart| match coverart {
                CoverartResponse::Json(coverart) => coverart
                    .images
                    .into_iter()
                    .find_map(|ca| ca.thumbnails.res_250),
                CoverartResponse::Url(url) => Some(url),
            })
            .filter(|coverart| coverart.ends_with(".jpg") || coverart.ends_with(".jpeg"));

        let relations = release.relations.unwrap_or_default();
        let wikidata_id: Option<String> = extract_relation_id(&relations, "wikidata");
        let discogs_id: Option<String> = extract_relation_id(&relations, "discogs");

        let mut metadata = ReleaseMetadata {
            mbid: release_mbid.to_string(),
            name: release.title,
            artist_mbid: artists_credited.first().map(|a| a.mbid.clone()),
            description: None,
            cover_art_url,
            artists_credited,
        };

        if let Some(discogs_id) = discogs_id {
            let release = self.get_discogs_release(&discogs_id).await?;

            metadata.cover_art_url = metadata.cover_art_url.or(release.thumb).or_else(|| {
                release
                    .images
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|image| image.r#type == "primary")
                    .filter(|image| !image.resource_url.is_empty())
                    .map(|image| image.resource_url)
                    .next()
            });
        }

        if let Some(wikidata_id) = wikidata_id {
            let item = self.get_wikidata(&wikidata_id).await?;
            let wiki_title = item
                .entities
                .get(&wikidata_id)
                .and_then(|e| e.sitelinks.get("enwiki"))
                .map(|d| d.title.clone());

            if let Some(title) = wiki_title {
                let description = self.get_wikipedia_description(&title).await?;
                metadata.description = Some(description);
            }
        }

        Ok(metadata)
    }
}

fn extract_relation_id(rels: &[Relation], relation_type: &str) -> Option<String> {
    rels.iter()
        .find(|rel| rel.relation_type == relation_type)
        .and_then(|rel| match &rel.content {
            RelationContent::Url(url) => url.resource.rsplit_once("/").map(|(_, id)| id.into()),
            _ => None,
        })
}

#[cfg(test)]
mod test {
    use crate::metadata::MetadataClient;
    use musicbrainz_rs::entity::release::Release;
    use musicbrainz_rs::{Fetch, FetchCoverart};

    #[tokio::test]
    async fn test_get_item() {
        let client = MetadataClient::default();
        let metadata = client
            .get_artist_metadata("381086ea-f511-4aba-bdf9-71c753dc5077")
            .await
            .unwrap();
        println!("{:?}", metadata);
    }

    #[tokio::test]
    async fn release() {
        let metadata = Release::fetch()
            .id("7639890c-70ad-49c3-8b2f-46da97724e2c")
            .execute()
            .await
            .unwrap();
        let ca = metadata
            .get_coverart()
            .front()
            .res_250()
            .execute()
            .await
            .unwrap();
        println!("{:?}", ca);
    }
}
