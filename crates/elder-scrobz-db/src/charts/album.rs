use crate::Period;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::types::chrono::{DateTime, Utc};
use std::result::Result;
use utoipa::ToSchema;

const DOCS: &str = include_str!("../../docs/album_charts.example.json");

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
#[schema(example = json!(DOCS))]
pub struct TopAlbum {
    pub release_id: String,
    pub release_name: String,
    pub cover_art_url: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
}

pub async fn get_most_listened_albums(
    period: Period,
    username: Option<String>,
    pool: &PgPool,
) -> Result<Vec<TopAlbum>, sqlx::Error> {
    let result = match username {
        None => match period {
            Period::Week => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/week.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/month.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/year.sql")
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/today.sql")
                    .fetch_all(pool)
                    .await?
            }
        },
        Some(user) => match period {
            Period::Week => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/user_week.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/user_month.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/user_year.sql", user)
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/user_today.sql", user)
                    .fetch_all(pool)
                    .await?
            }
        },
    };

    Ok(result)
}
