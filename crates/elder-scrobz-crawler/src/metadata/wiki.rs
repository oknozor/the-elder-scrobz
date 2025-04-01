use crate::metadata::MetadataClient;
use serde::Deserialize;
use std::collections::HashMap;

impl MetadataClient {
    pub async fn get_wikidata(&self, id: &str) -> Result<WikidataPayload, reqwest::Error> {
        let value = self
            .client
            .get(format!(
                "https://www.wikidata.org/w/api.php?action=wbgetentities&ids={}&format=json&props=descriptions|claims&languages=en",
                id
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(value)
    }
}

#[derive(Debug, Deserialize)]
pub struct WikidataPayload {
    pub entities: HashMap<String, WikidataEntity>,
}

#[derive(Debug, Deserialize)]
pub struct WikidataEntity {
    pub descriptions: HashMap<String, Description>,
    pub claims: HashMap<String, Vec<Claim>>,
}

#[derive(Debug, Deserialize)]
pub struct Description {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Claim {
    pub mainsnak: MainSnak,
}

#[derive(Debug, Deserialize)]
pub struct MainSnak {
    pub datavalue: Option<DataValue>,
}

#[derive(Debug, Deserialize)]
pub struct DataValue {
    pub value: String,
}
