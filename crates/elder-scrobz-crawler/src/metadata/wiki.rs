use crate::metadata::MetadataClient;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

impl MetadataClient {
    pub async fn get_wikidata(&self, id: &str) -> Result<WikidataPayload, reqwest::Error> {
        self.client
            .get(format!(
                "https://www.wikidata.org/w/api.php?action=wbgetentities&ids={id}&format=json&props=sitelinks&languages=en",
            ))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_wikipedia_description(
        &self,
        title: &str,
    ) -> Result<Option<String>, reqwest::Error> {
        let value: Value = self
            .client
            .get(format!(
                "https://en.wikipedia.org/api/rest_v1/page/summary/{title}"
            ))
            .send()
            .await?
            .json()
            .await?;

        let value = value["extract"].as_str().map(ToString::to_string);
        Ok(value)
    }
}

#[derive(Debug, Deserialize)]
pub struct WikidataPayload {
    pub entities: HashMap<String, WikidataEntity>,
}

#[derive(Debug, Deserialize)]
pub struct WikidataEntity {
    pub sitelinks: HashMap<String, SiteLink>,
}

#[derive(Debug, Deserialize)]
pub struct SiteLink {
    pub site: String,
    pub title: String,
}
