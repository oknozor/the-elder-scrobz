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
    pub oauth_introspection_url: String,
}

impl Settings {
    pub(crate) fn get() -> Result<Self, config::ConfigError> {
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

    pub fn coverart_url(&self, release_id: &str) -> Option<String> {
        let release_ca = format!("{release_id}.jpg");
        let coverart_path = self.coverart_path.join(release_ca);
        if !coverart_path.exists() {
            return None;
        };

        Some(format!("/coverarts/{release_id}.jpg"))
    }
}
