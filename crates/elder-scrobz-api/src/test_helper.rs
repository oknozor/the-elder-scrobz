use crate::oauth::client::Oauth2Client;
use crate::settings::Settings;
use crate::state::AppState;
use elder_scrobz_db::{build_pg_pool, migrate_db};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::ContainerAsync;

pub fn scrobble_fixture() -> anyhow::Result<String> {
    let path = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = PathBuf::from(path).join("tests/fixtures/scrobble.json");
    let scrobble = fs::read_to_string(path)?;
    Ok(scrobble)
}

pub async fn start_postgres() -> anyhow::Result<(AppState, ContainerAsync<Postgres>)> {
    let container = Postgres::default()
        .with_db_name("scrobz")
        .with_user("scrobz")
        .with_password("scrobz")
        .start()
        .await?;

    let host_port = container.get_host_port_ipv4(5432).await?;
    let host_ip = container.get_host().await?;
    let admin_db_url = format!("postgres://scrobz:scrobz@{host_ip}:{host_port}/scrobz");

    let pool = build_pg_pool(&admin_db_url).await;

    let settings = Arc::new(Settings::get()?);

    migrate_db(&pool).await?;
    Ok((
        AppState {
            pool,
            settings,
            oauth_client: Oauth2Client::noop(),
        },
        container,
    ))
}
