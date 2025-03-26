use crate::Period;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::types::chrono::{DateTime, Utc};
use std::result::Result;
use utoipa::ToSchema;

const DOCS: &str = include_str!("../../docs/track_charts.example.json");

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
#[schema(example = json!(DOCS))]
pub struct TopTrack {
    pub track_id: String,
    pub track_name: String,
    pub track_length: Option<i32>,
    pub release_name: String,
    pub cover_art_url: Option<String>,
    pub listened_at: DateTime<Utc>,
    pub listens: Option<i64>,
}

pub async fn get_most_listened_tracks(
    period: Period,
    username: Option<String>,
    pool: &PgPool,
) -> Result<Vec<TopTrack>, sqlx::Error> {
    let result = match username {
        None => match period {
            Period::Week => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/week.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/month.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/year.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/today.sql")
                    .fetch_all(pool)
                    .await?
            }
        },
        Some(user) => match period {
            Period::Week => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/user_week.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/user_month.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/user_year.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/user_today.sql", user)
                    .fetch_all(pool)
                    .await?
            }
        },
    };

    Ok(result)
}
