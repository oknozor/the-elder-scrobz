use crate::Period;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::types::chrono::{DateTime, Utc};
use std::result::Result;
use utoipa::ToSchema;

const DOCS: &str = include_str!("../../docs/artist_charts.example.json");

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
#[schema(example = json!(DOCS))]
pub struct TopArtist {
    pub artist_id: String,
    pub artist_name: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
}

pub async fn get_most_listened_artists(
    period: Period,
    username: Option<String>,
    pool: &PgPool,
) -> Result<Vec<TopArtist>, sqlx::Error> {
    let result = match username {
        None => match period {
            Period::Week => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/week.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/month.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/year.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/today.sql")
                    .fetch_all(pool)
                    .await?
            }
        },
        Some(user) => match period {
            Period::Week => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/user_week.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/user_month.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/user_year.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/user_today.sql", user)
                    .fetch_all(pool)
                    .await?
            }
        },
    };

    Ok(result)
}
