use crate::metadata::MetadataClient;
use serde::Deserialize;

impl MetadataClient {
    pub async fn get_discogs_artist(&self, id: &str) -> Result<DiscogsArtist, reqwest::Error> {
        let artist = self
            .client
            .get(format!("https://api.discogs.com/artists/{}", id))
            .header("User-Agent", "MyDiscogsClient/1.0")
            .send()
            .await?
            .json()
            .await?;

        Ok(artist)
    }
}

#[derive(Debug, Deserialize)]
pub struct DiscogsArtist {
    pub id: i64,
    pub name: String,
    pub realname: Option<String>,
    pub profile: String,
    pub urls: Vec<String>,
    pub images: Option<Vec<DiscogsImage>>,
}

#[derive(Debug, Deserialize)]
pub struct DiscogsImage {
    pub height: i32,
    pub width: i32,
    pub uri: String,
    pub resource_url: String,
    pub uri150: String,
    pub r#type: String,
}
