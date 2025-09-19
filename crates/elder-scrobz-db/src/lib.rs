pub mod api_key;
pub mod charts;
pub mod dlc;
pub mod listens;
pub mod pulses;
pub mod stats;
pub mod user;

use serde::{Deserialize, Serialize};
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

#[derive(Debug, ToSchema, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
#[schema(default = "year", example = "month")]
pub enum Period {
    Week,
    Month,
    #[default]
    Year,
    Today,
    All,
}

#[derive(Debug)]
pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
}

impl From<(i64, i64)> for Pagination {
    fn from((limit, offset): (i64, i64)) -> Self {
        Self { limit, offset }
    }
}
