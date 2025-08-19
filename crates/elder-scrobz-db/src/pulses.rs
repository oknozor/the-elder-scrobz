use crate::Period;
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
pub struct Pulse {
    pub listens: Option<i64>,
    pub period: Option<String>,
}

impl Pulse {
    pub async fn for_period(
        period: Period,
        user_id: Option<String>,
        pool: &PgPool,
    ) -> Result<Vec<Pulse>, sqlx::Error> {
        let result = match user_id {
            None => match period {
                Period::Week => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/week.sql")
                        .fetch_all(pool)
                        .await?
                }
                Period::Month => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/month.sql")
                        .fetch_all(pool)
                        .await?
                }
                Period::Year => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/year.sql")
                        .fetch_all(pool)
                        .await?
                }
                Period::Today => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/today.sql")
                        .fetch_all(pool)
                        .await?
                }
                Period::All => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/all.sql")
                        .fetch_all(pool)
                        .await?
                }
            },
            Some(user) => match period {
                Period::Week => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/user_week.sql", user)
                        .fetch_all(pool)
                        .await?
                }
                Period::Month => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/user_month.sql", user)
                        .fetch_all(pool)
                        .await?
                }
                Period::Year => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/user_year.sql", user)
                        .fetch_all(pool)
                        .await?
                }
                Period::Today => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/user_today.sql", user)
                        .fetch_all(pool)
                        .await?
                }
                Period::All => {
                    sqlx::query_file_as!(Pulse, "queries/pulses/user_all.sql", user)
                        .fetch_all(pool)
                        .await?
                }
            },
        };

        Ok(result)
    }
}
