use serde::{Deserialize, Serialize};
use sqlx::Executor;
use sqlx::FromRow;
use sqlx::PgPool;
use sqlx::Postgres;
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UserConfig {
    pub username: String,
    pub enable_weekly_playlist: bool,
    pub enable_monthly_playlist: bool,
    pub enable_yearly_playlist: bool,
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct GlobalConfig {
    pub id: i32,
    pub enable_weekly_playlist: bool,
    pub enable_monthly_playlist: bool,
    pub enable_yearly_playlist: bool,
}

impl UserConfig {
    pub async fn get(
        pool: impl Executor<'_, Database = Postgres>,
        username: &str,
    ) -> Result<UserConfig, sqlx::Error> {
        let user_config = sqlx::query_as!(
            UserConfig,
            "SELECT * FROM user_configs WHERE username = $1",
            username
        )
        .fetch_one(pool)
        .await?;
        Ok(user_config)
    }

    pub async fn save(
        &self,
        pool: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query_as!(UserConfig, r#"INSERT INTO user_configs (username, enable_weekly_playlist, enable_monthly_playlist, enable_yearly_playlist) VALUES ($1, $2, $3, $4)
                             ON CONFLICT (username) DO UPDATE SET enable_weekly_playlist = $2, enable_monthly_playlist = $3, enable_yearly_playlist = $4"#,
                             self.username,
                             self.enable_weekly_playlist,
                             self.enable_monthly_playlist,
                             self.enable_yearly_playlist

        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl GlobalConfig {
    pub async fn get(pool: &PgPool) -> Result<GlobalConfig, sqlx::Error> {
        let global_config =
            sqlx::query_as!(GlobalConfig, "SELECT * FROM global_config WHERE id = 1")
                .fetch_one(pool)
                .await?;
        Ok(global_config)
    }

    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            GlobalConfig,
            r#"INSERT INTO global_config (id, enable_weekly_playlist, enable_monthly_playlist, enable_yearly_playlist) VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET enable_weekly_playlist = $2, enable_monthly_playlist = $3, enable_yearly_playlist = $4"#,
            self.id,
            self.enable_weekly_playlist,
            self.enable_monthly_playlist,
            self.enable_yearly_playlist
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
