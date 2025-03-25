pub mod api_key;
pub mod charts;
pub mod listens;
pub mod pulses;
pub mod user;

use serde::Deserialize;
pub use sqlx::postgres::{PgPool, PgPoolOptions};
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

#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Week,
    Month,
    Year,
    Today,
}
