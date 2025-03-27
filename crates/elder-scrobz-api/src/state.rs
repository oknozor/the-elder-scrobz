use crate::oauth::client::{get_oauth2_client, Oauth2Client};
use crate::settings::Settings;
use elder_scrobz_db::{build_pg_pool, PgPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub settings: Arc<Settings>,
    pub oauth_client: Oauth2Client,
}

impl AppState {
    pub async fn init() -> anyhow::Result<Self> {
        let settings = Settings::get()?;
        let pool = build_pg_pool(&settings.database_url).await;
        let settings = Arc::new(settings);
        let oauth_client = get_oauth2_client(&settings).await?;
        Ok(AppState {
            pool,
            settings,
            oauth_client,
        })
    }
}
