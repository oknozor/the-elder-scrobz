pub mod api_key;
pub mod charts;
pub mod listens;
pub mod pulses;
pub mod stats;
pub mod user;

use serde::{Deserialize, Serialize};
pub use sqlx::postgres::{PgPool, PgPoolOptions};
use std::path::Path;
use std::time::Duration;
use utoipa::ToSchema;

pub async fn build_pg_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
        .expect("can't connect to database")
}

pub async fn migrate_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}

pub trait WithLocalImage {
    fn with_local_image(mut self, base_path: &Path) -> Self
    where
        Self: Sized,
    {
        let mbid = self.mbid();
        let image = format!("{mbid}.jpg");
        let coverart_path = base_path.join(image);
        if !coverart_path.exists() {
            self
        } else {
            self.set_image_path(format!("/coverarts/{mbid}.jpg"));
            self
        }
    }

    fn set_image_path(&mut self, path: String);
    fn mbid(&self) -> &str;
}

#[derive(Debug, ToSchema, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
#[schema(default = "year", example = "month")]
pub enum Period {
    Week,
    Month,
    #[default]
    Year,
    Today,
}
