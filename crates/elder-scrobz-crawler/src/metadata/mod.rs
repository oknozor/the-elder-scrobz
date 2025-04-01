
mod discogs;
mod wiki;

pub struct MetadataClient {
    client: reqwest::Client,
}

impl MetadataClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[cfg(test)]
mod test {
    
    use crate::metadata::MetadataClient;
    use musicbrainz_rs::entity::relations::{Relation, RelationContent};
    use musicbrainz_rs::{Fetch, entity::artist::Artist};

    #[tokio::test]
    async fn test_get_item() {
        let nirvana = Artist::fetch()
            .id("5b11f4ce-a62d-471e-81fc-a69a8278c7da")
            .with_url_relations()
            .execute()
            .await
            .unwrap();

        let relations = nirvana.relations.unwrap_or_default();
        let get_relation_id = |rels: &[Relation], relation_type: &str| {
            rels.iter()
                .find(|rel| rel.relation_type == relation_type)
                .and_then(|rel| match &rel.content {
                    RelationContent::Url(url) => {
                        url.resource.rsplit_once("/").map(|(_, id)| id.into())
                    }
                    _ => None,
                })
        };

        let wikidata_id: Option<String> = get_relation_id(&relations, "wikidata");
        let discogs_id: Option<String> = get_relation_id(&relations, "discogs");

        println!("{:?}", wikidata_id);
        println!("{:?}", discogs_id);
        let client = MetadataClient::new();
        let artist = client
            .get_discogs_artist(&discogs_id.unwrap())
            .await
            .unwrap();
        println!("Artist: {:?}", artist);
        let wikidata_id = wikidata_id.unwrap();
        let item = client.get_wikidata(&wikidata_id).await.unwrap();
        println!("{:?}", item);

        // let description = item["entities"][&wikidata_id]["descriptions"]["en"]["value"]
        //     .as_str()
        //     .unwrap_or("No description available");
        //
        // // Extract image
        // let image_filename =
        //     item["entities"][&wikidata_id]["claims"]["P18"][0]["mainsnak"]["datavalue"]["value"]
        //         .as_str();
        //
        // let image_url = match image_filename {
        //     Some(filename) => format!(
        //         "https://commons.wikimedia.org/wiki/Special:FilePath/{}",
        //         filename
        //     ),
        //     None => "No image available".to_string(),
        // };

        // println!("{}", description);
        // println!("{:?}", image_filename);
        // println!("{}", image_url);
    }
}
