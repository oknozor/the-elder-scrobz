use crate::charts::Period;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::types::chrono::{DateTime, Utc};
use std::result::Result;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
pub struct TopTrack {
    pub track_id: String,
    pub track_name: String,
    pub track_length: Option<i32>,
    pub release_name: String,
    pub cover_art_url: Option<String>,
    pub listened_at: DateTime<Utc>,
    pub listens: Option<i64>,
}

pub async fn get_most_listened_track(
    period: Period,
    pool: &PgPool,
) -> Result<Vec<TopTrack>, sqlx::Error> {
    let result = match period {
        Period::Week => {
            sqlx::query_file_as!(TopTrack, "queries/track_chart_week.sql")
                .fetch_all(pool)
                .await?
        }
        Period::Month => {
            sqlx::query_file_as!(TopTrack, "queries/track_chart_month.sql")
                .fetch_all(pool)
                .await?
        }
        Period::Year => {
            sqlx::query_file_as!(TopTrack, "queries/track_chart_year.sql")
                .fetch_all(pool)
                .await?
        }
        Period::Today => {
            sqlx::query_file_as!(TopTrack, "queries/track_chart_today.sql")
                .fetch_all(pool)
                .await?
        }
    };

    Ok(result)
}
