use elder_scrobz_settings::Settings;
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenResponse, BasicTokenType,
};
use oauth2::{
    AccessToken, ClientId, ClientSecret, EndpointNotSet, EndpointSet, IntrospectionUrl,
    StandardRevocableToken, StandardTokenIntrospectionResponse,
};
use reqwest::Client;
use serde::Deserialize;
use tracing::info;

use crate::oauth::ExtraClaim;

type IntrospectionResponse = StandardTokenIntrospectionResponse<ExtraClaim, BasicTokenType>;

type Oauth2IntrospectClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    IntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
>;

pub type ScrobzOauth2Client<
    HasAuthUrl = EndpointNotSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointNotSet,
    HasRevocationUrl = EndpointNotSet,
    HasTokenUrl = EndpointNotSet,
> = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    IntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    HasAuthUrl,
    HasDeviceAuthUrl,
    HasIntrospectionUrl,
    HasRevocationUrl,
    HasTokenUrl,
>;

#[derive(Debug, Deserialize)]
struct OAuthMetadata {
    introspection_endpoint: String,
}

#[derive(Clone, Debug)]
pub struct Oauth2Client {
    client: Client,
    oauth2_client: Oauth2IntrospectClient,
}

impl Oauth2Client {
    pub(crate) async fn introspect(&self, token: String) -> anyhow::Result<IntrospectionResponse> {
        let token = AccessToken::new(token);
        self.oauth2_client
            .introspect(&token)
            .request_async(&self.client)
            .await
            .map_err(Into::into)
    }
}

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

    let oauth2_client = ScrobzOauth2Client::new(client_id)
        .set_client_secret(client_secret)
        .set_introspection_url(introspection);

    info!("Oauth2 client ready");
    Ok(Oauth2Client {
        client,
        oauth2_client,
    })
}
