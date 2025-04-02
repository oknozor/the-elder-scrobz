use crate::Period;
use serde::Serialize;
use sqlx::PgPool;
use std::result::Result;
use utoipa::ToSchema;

const DOCS: &str = include_str!("../../docs/album_charts.example.json");

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
#[schema(example = json!(DOCS))]
pub struct Overview {
    pub artist_listened: Option<i64>,
    pub track_listened: Option<i64>,
    pub time_listened: Option<i64>,
    pub artist_listened_percentage_increase: Option<f64>,
    pub track_listened_percentage_increase: Option<f64>,
    pub time_listened_percentage_increase: Option<f64>,
}

pub async fn get_overview(
    period: Period,
    pool: &PgPool,
) -> Result<Overview, sqlx::Error> {
    let result = match period {
        Period::Week => {
            sqlx::query_file_as!(Overview, "queries/charts/overview/week.sql")
                .fetch_one(pool)
                .await?
        }
        Period::Month => {
            sqlx::query_file_as!(Overview, "queries/charts/overview/month.sql")
                .fetch_one(pool)
                .await?
        }
        Period::Year => {
            sqlx::query_file_as!(Overview, "queries/charts/overview/year.sql")
                .fetch_one(pool)
                .await?
        }
        Period::Today => {
            sqlx::query_file_as!(Overview, "queries/charts/overview/today.sql")
                .fetch_one(pool)
                .await?
        }
    };

    Ok(result)
}
