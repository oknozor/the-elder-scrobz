use crate::settings::Settings;
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AccessToken, ClientId, ClientSecret, EndpointNotSet, EndpointSet, IntrospectionUrl,
    StandardRevocableToken,
};
use reqwest::Client;
use serde::Deserialize;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Oauth2Client {
    client: Client,
    oauth2_client: Oauth2IntrospectClient,
}

impl Oauth2Client {
    pub(crate) async fn introspect(
        &self,
        token: String,
    ) -> anyhow::Result<BasicTokenIntrospectionResponse> {
        let token = AccessToken::new(token);
        self.oauth2_client
            .introspect(&token)
            .request_async(&self.client)
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Deserialize)]
struct OAuthMetadata {
    introspection_endpoint: String,
}

type Oauth2IntrospectClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
>;

pub async fn get_oauth2_client(settings: &Settings) -> anyhow::Result<Oauth2Client> {
    info!("Fetching oauth2 provider metadata");
    let client = Client::new();
    let metadata: OAuthMetadata = client
        .get(&settings.oauth_provider_url)
        .send()
        .await?
        .json()
        .await?;

    let client_id = ClientId::new(settings.oauth_client_id.clone());
    let client_secret = ClientSecret::new(settings.oauth_client_secret.clone());
    let introspection = IntrospectionUrl::new(metadata.introspection_endpoint)?;

    let oauth2_client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_introspection_url(introspection);

    info!("Oauth2 client ready");
    Ok(Oauth2Client {
        client,
        oauth2_client,
    })
}

#[cfg(test)]
pub mod fixture {
    use crate::oauth::client::Oauth2Client;
    use oauth2::basic::BasicClient;
    use oauth2::{ClientId, ClientSecret, IntrospectionUrl};

    impl Oauth2Client {
        pub fn noop() -> Self {
            let client_id = ClientId::new("noop".to_string());
            let client_secret = ClientSecret::new("noop".to_string());
            let introspection = IntrospectionUrl::new("https://noop.com".to_string()).unwrap();
            let client = BasicClient::new(client_id)
                .set_client_secret(client_secret)
                .set_introspection_url(introspection);

            Self {
                client: Default::default(),
                oauth2_client: client,
            }
        }
    }
}
