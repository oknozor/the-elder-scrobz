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
    pub id: String,
    pub name: String,
    pub subsonic_id: Option<String>,
    pub thumbnail_url: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
    pub year: Option<i32>,
    #[serde(skip)]
    pub(crate) total: Option<i64>,
}

pub async fn get_most_listened_albums(
    period: Period,
    username: Option<String>,
    limit: i64,
    offset: i64,
    pool: &PgPool,
) -> Result<(i64, Vec<TopAlbum>), sqlx::Error> {
    let result = match username {
        None => match period {
            Period::Week => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/week.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/month.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/year.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopAlbum, "queries/charts/album/today.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::All => todo!(),
        },
        Some(user) => match period {
            Period::Week => {
                sqlx::query_file_as!(
                    TopAlbum,
                    "queries/charts/album/user_week.sql",
                    user,
                    limit,
                    offset
                )
                .fetch_all(pool)
                .await?
            }
            Period::Month => {
                sqlx::query_file_as!(
                    TopAlbum,
                    "queries/charts/album/user_month.sql",
                    user,
                    limit,
                    offset
                )
                .fetch_all(pool)
                .await?
            }
            Period::Year => {
                sqlx::query_file_as!(
                    TopAlbum,
                    "queries/charts/album/user_year.sql",
                    user,
                    limit,
                    offset
                )
                .fetch_all(pool)
                .await?
            }
            Period::Today => {
                sqlx::query_file_as!(
                    TopAlbum,
                    "queries/charts/album/user_today.sql",
                    user,
                    limit,
                    offset
                )
                .fetch_all(pool)
                .await?
            }
            Period::All => todo!(),
        },
    };

    let total = result.first().and_then(|r| r.total).unwrap_or_default();
    Ok((total, result))
}
