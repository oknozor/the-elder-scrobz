use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, ToSchema)]
pub struct ErroredScrobble {
    pub id: i64,
    pub user_id: String,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

pub struct CreateErroredScrobble {
    pub user_id: String,
    pub data: serde_json::Value,
}

impl CreateErroredScrobble {
    pub async fn save(&self, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO scrobbles_errored (user_id, data)
                VALUES ($1, $2)
            "#,
            self.user_id,
            self.data
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl ErroredScrobble {
    pub async fn all(
        pool: &sqlx::PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
                SELECT id, user_id, data, created_at
                FROM scrobbles_errored
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await
    }
}
