use serde::Deserialize;
use submarine::{auth::AuthBuilder, data::SearchResult3};

#[derive(Debug, Clone)]
pub struct SubsonicClient {
    inner: submarine::Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubsonicConfig {
    pub username: String,
    pub password: String,
    pub server_url: String,
}

impl SubsonicClient {
    pub fn new(config: SubsonicConfig) -> Self {
        Self {
            inner: submarine::Client::new(
                &config.server_url,
                AuthBuilder::new(&config.username, "v16.1.0")
                    .client_name("the-edler-scrobz")
                    .hashed(&config.password),
            ),
        }
    }

    pub async fn search_by_mbid(&self, mbid: &str) -> Result<SearchResult3, anyhow::Error> {
        self.inner
            .search3(mbid, None, None, None, None, None, None, None::<String>)
            .await
            .map_err(Into::into)
    }

    pub async fn create_playlist(
        &self,
        name: &str,
        tracks: Vec<String>,
    ) -> Result<(), anyhow::Error> {
        self.inner.create_playlist(name, tracks).await?;
        Ok(())
    }
}
