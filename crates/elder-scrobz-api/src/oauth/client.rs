use elder_scrobz_settings::OidcConfig;
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AuthUrl, ClientSecret, EndpointNotSet, EndpointSet, IntrospectionUrl, RedirectUrl,
    StandardRevocableToken, TokenIntrospectionResponse, TokenResponse, TokenUrl,
};
use oauth2::{AuthorizationCode, ClientId};
use reqwest::Client;
use serde::Deserialize;

pub type ScrobzOauth2Client2<
    HasAuthUrl = EndpointSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointSet,
    HasRevocationUrl = EndpointNotSet,
    HasTokenUrl = EndpointSet,
> = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    HasAuthUrl,
    HasDeviceAuthUrl,
    HasIntrospectionUrl,
    HasRevocationUrl,
    HasTokenUrl,
>;

#[derive(Debug, Clone)]
pub struct OauthClient {
    pub oauth: ScrobzOauth2Client2,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub struct WellKnownConfiguration {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub introspection_endpoint: String,
}

impl OauthClient {
    pub async fn build(config: &OidcConfig) -> anyhow::Result<Self> {
        let client_id = config.client_id.to_string();
        let client_secret = config.client_secret.to_string();
        let redirect_url = format!(
            "{}://{}/auth/authorized",
            if config.force_http { "http" } else { "https" },
            config.domain
        );

        let configuration: WellKnownConfiguration = reqwest::get(&format!(
            "{}/.well-known/openid-configuration",
            config.provider_url
        ))
        .await?
        .json()
        .await?;

        let oauth = ScrobzOauth2Client2::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(AuthUrl::new(configuration.authorization_endpoint)?)
            .set_token_uri(TokenUrl::new(configuration.token_endpoint)?)
            .set_introspection_url(IntrospectionUrl::new(configuration.introspection_endpoint)?)
            .set_redirect_uri(RedirectUrl::new(redirect_url)?);

        Ok(Self {
            oauth,
            http: Client::default(),
        })
    }

    pub async fn get_token(&self, code: String) -> anyhow::Result<String> {
        let token = self
            .oauth
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&self.http)
            .await?;

        let introspection = self
            .oauth
            .introspect(token.access_token())
            .request_async(&self.http)
            .await?;

        introspection
            .sub()
            .or(introspection.username())
            .map(ToString::to_string)
            .ok_or_else(|| anyhow::anyhow!("failed to get username from introsection"))
    }
}
