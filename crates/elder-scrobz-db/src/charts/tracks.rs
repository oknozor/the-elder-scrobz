use crate::{Period, WithLocalImage};
use serde::Serialize;
use sqlx::PgPool;
use std::result::Result;
use utoipa::ToSchema;

const DOCS: &str = include_str!("../../docs/track_charts.example.json");

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
#[schema(example = json!(DOCS))]
pub struct TopTrack {
    pub id: String,
    pub name: String,
    pub length: Option<i32>,
    pub release_mbid: String,
    pub release_name: String,
    pub cover_art_url: Option<String>,
    pub listens: Option<i64>,
    #[serde(skip)]
    pub total: Option<i64>,
}

impl WithLocalImage for TopTrack {
    fn mbid(&self) -> &str {
        &self.release_mbid
    }

    fn set_image_path(&mut self, path: String) {
        self.cover_art_url = Some(path);
    }
}

pub async fn get_most_listened_tracks(
    period: Period,
    username: Option<String>,
    limit: i64,
    offset: i64,
    pool: &PgPool,
) -> Result<(i64, Vec<TopTrack>), sqlx::Error> {
    let result = match username {
        None => match period {
            Period::Week => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/week.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Month => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/month.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Year => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/year.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::Today => {
                sqlx::query_file_as!(TopTrack, "queries/charts/track/today.sql", limit, offset)
                    .fetch_all(pool)
                    .await?
            }
            Period::All => todo!(),
        },
        Some(user) => match period {
            Period::Week => {
                sqlx::query_file_as!(
                    TopTrack,
                    "queries/charts/track/user_week.sql",
                    user,
                    limit,
                    offset,
                )
                .fetch_all(pool)
                .await?
            }
            Period::Month => {
                sqlx::query_file_as!(
                    TopTrack,
                    "queries/charts/track/user_month.sql",
                    user,
                    limit,
                    offset,
                )
                .fetch_all(pool)
                .await?
            }
            Period::Year => {
                sqlx::query_file_as!(
                    TopTrack,
                    "queries/charts/track/user_year.sql",
                    user,
                    limit,
                    offset,
                )
                .fetch_all(pool)
                .await?
            }
            Period::Today => {
                sqlx::query_file_as!(
                    TopTrack,
                    "queries/charts/track/user_today.sql",
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

    let total = result.first().and_then(|t| t.total).unwrap_or_default();
    Ok((total, result))
}
