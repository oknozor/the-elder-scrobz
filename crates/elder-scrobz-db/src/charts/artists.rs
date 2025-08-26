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
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub subsonic_id: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
    #[serde(skip)]
    pub total: Option<i64>,
}

pub async fn get_most_listened_artists(
    period: Period,
    username: Option<String>,
    limit: i64,
    offset: i64,
    pool: &PgPool,
) -> Result<(i64, Vec<TopArtist>), sqlx::Error> {
    let result = match username {
        None => match period {
            Period::Week => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/week.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/month.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/year.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopArtist, "queries/charts/artist/today.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::All => todo!(),
        },
        Some(user) => match period {
            Period::Week => {
                sqlx::query_file_as!(
                    TopArtist,
                    "queries/charts/artist/user_week.sql",
                    user,
                    limit,
                    offset,
                )
                .fetch_all(pool)
                .await?
            }
            Period::Month => {
                sqlx::query_file_as!(
                    TopArtist,
                    "queries/charts/artist/user_month.sql",
                    user,
                    limit,
                    offset,
                )
                .fetch_all(pool)
                .await?
            }
            Period::Year => {
                sqlx::query_file_as!(
                    TopArtist,
                    "queries/charts/artist/user_year.sql",
                    user,
                    limit,
                    offset,
                )
                .fetch_all(pool)
                .await?
            }
            Period::Today => {
                sqlx::query_file_as!(
                    TopArtist,
                    "queries/charts/artist/user_today.sql",
                    user,
                    limit,
                    offset,
                )
                .fetch_all(pool)
                .await?
            }
            Period::All => todo!(),
        },
    };

    let total = result.first().and_then(|a| a.total).unwrap_or_default();
    Ok((total, result))
}
