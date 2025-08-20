use crate::metadata::MetadataClient;
use serde::Deserialize;

const USER_AGENT: &str = "TheElderScrobz/1.0";

impl MetadataClient {
    pub async fn get_discogs_artist(&self, id: &str) -> Result<DiscogsArtist, reqwest::Error> {
        let artist = self
            .client
            .get(format!("https://api.discogs.com/artists/{id}"))
            .header("User-Agent", USER_AGENT)
            .header(
                "Authorization",
                &format!("Discogs token={}", self.discogs_token),
            )
            .send()
            .await?
            .json()
            .await?;

        Ok(artist)
    }

    pub async fn get_discogs_release(&self, id: &str) -> Result<DiscogsRelease, reqwest::Error> {
        let artist = self
            .client
            .get(format!("https://api.discogs.com/releases/{id}"))
            .header("User-Agent", USER_AGENT)
            .header(
                "Authorization",
                &format!("Discogs token={}", self.discogs_token),
            )
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

#[cfg(test)]
mod test {
    use crate::MetadataClient;

    #[tokio::test]
    async fn test_get_discogs_release() {
        let client = MetadataClient::new("rzHdPLOtMKbqKltijMsrhxDHFwTZvvWoFvLcXJjR".to_string());
        let release = client
            .get_release_metadata("bb772ff7-7ed8-435b-9bfe-90df819fa605")
            .await;
        println!("{:?}", release);
    }
}
