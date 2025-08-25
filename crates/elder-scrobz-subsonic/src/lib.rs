use anyhow::anyhow;
use reqwest::Client;
use serde::Deserialize;
use subsonic_types::{
    common::Version,
    request::{Authentication, Request, SubsonicRequest},
    response::{Response, ResponseBody, SearchResult3},
};

#[derive(Debug, Clone)]
pub struct SubsonicClient {
    inner: Client,
    config: SubsonicConfig,
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
            inner: Client::new(),
            config,
        }
    }

    fn config(&self) -> &SubsonicConfig {
        &self.config
    }

    fn build_request<R>(&self, body: R) -> String
    where
        R: SubsonicRequest,
    {
        let request = Request {
            username: self.config().username.clone(),
            authentication: Authentication::Password(self.config().password.clone()),
            version: Version::LATEST,
            client: "ratatune".into(),
            format: Some("json".into()),
            body,
        };

        format!(
            "{}{}?{}",
            &self.config().server_url,
            R::PATH,
            request.to_query()
        )
    }

    async fn make_request<R>(&self, body: R) -> Result<reqwest::Response, reqwest::Error>
    where
        R: SubsonicRequest,
    {
        let url = self.build_request(body);
        self.inner
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
    }

    pub async fn search_by_mbid(&self, mbid: &str) -> Result<SearchResult3, anyhow::Error> {
        let request = subsonic_types::request::search::Search3 {
            query: mbid.to_string(),
            artist_count: None,
            artist_offset: None,
            album_count: None,
            album_offset: None,
            song_count: None,
            song_offset: None,
            music_folder_id: None,
        };
        let response = self
            .make_request(request)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let response = Response::from_json(&response).unwrap();

        let ResponseBody::SearchResult3(response) = response.body else {
            return Err(anyhow!("Failed to get search results"));
        };
        Ok(response)
    }
}
