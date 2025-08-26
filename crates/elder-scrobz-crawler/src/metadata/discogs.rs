use crate::metadata::MetadataClient;
use anyhow::Context;
use serde::Deserialize;

const USER_AGENT: &str = "TheElderScrobz/1.0";

impl MetadataClient {
    pub async fn get_discogs_artist(&self, id: &str) -> anyhow::Result<DiscogsArtist> {
        let artist = self
            .client
            .get(format!("https://api.discogs.com/artists/{id}"))
            .header("User-Agent", USER_AGENT)
            .header(
                "Authorization",
                &format!(
                    "Discogs key={}, secret={}",
                    self.discogs_key, self.discogs_secret
                ),
            )
            .send()
            .await
            .context("Faild to send artist request to discog")?
            .json()
            .await
            .context("Failed to deserialize artist response from discog")?;

        Ok(artist)
    }

    pub async fn get_discogs_release(&self, id: &str) -> anyhow::Result<DiscogsRelease> {
        let release = self
            .client
            .get(format!("https://api.discogs.com/releases/{id}"))
            .header("User-Agent", USER_AGENT)
            .header(
                "Authorization",
                &format!(
                    "Discogs key={}, secret={}",
                    self.discogs_key, self.discogs_secret
                ),
            )
            .send()
            .await
            .context("Faild to send release request to discog")?
            .json()
            .await
            .context("Failed to deserialize release response from discog")?;

        Ok(release)
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
pub struct DiscogsRelease {
    pub id: i64,
    pub title: String,
    pub thumb: Option<String>,
    pub images: Option<Vec<DiscogsImage>>,
    pub year: u32,
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
