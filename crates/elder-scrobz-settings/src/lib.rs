use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub debug: bool,
    pub port: u16,
    pub coverart_path: PathBuf,
    pub database_url: String,
    pub oauth_client_id: String,
    pub oauth_client_secret: String,
    pub oauth_provider_url: String,
    pub discogs_token: String,
    pub navidrome_username: String,
    pub navidrome_password: String,
    pub navidrome_server_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubsonicConfig {
    pub username: String,
    pub password: String,
    pub server_url: String,
}

impl Settings {
    pub fn get() -> Result<Self, config::ConfigError> {
        let mut config = Config::builder().add_source(
            Environment::with_prefix("SCROBZ")
                .try_parsing(true)
                .prefix_separator("__")
                .separator("__"),
        );

        let etc_config = PathBuf::from("/etc/scrobz/config.toml");
        if etc_config.exists() {
            config = config.add_source(File::from(etc_config));
        }

        let default_config = PathBuf::from("config.toml");
        if default_config.exists() {
            config = config.add_source(File::from(default_config));
        }

        config.build()?.try_deserialize()
    }
}
