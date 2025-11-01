use crate::PgPool;
use crate::api_key::{key_sha, verify_api_key};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub admin: bool,
}

impl CreateUser {
    pub async fn insert(self, pool: &PgPool) -> Result<UserWithRole, Error> {
        let user = sqlx::query_as!(
            UserWithRole,
            r#"INSERT INTO users (username, admin) VALUES ($1, $2) returning username, admin"#,
            self.username,
            self.admin,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserWithRole {
    pub username: String,
    pub admin: bool,
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub username: String,
}

impl From<UserWithTotal> for User {
    fn from(value: UserWithTotal) -> Self {
        User {
            username: value.username,
        }
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserWithTotal {
    pub username: String,
    #[serde(skip)]
    pub total: Option<i64>,
}

impl User {
    pub async fn get_by_username_with_permission(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<UserWithRole>, Error> {
        let user = sqlx::query_as!(
            UserWithRole,
            r#"
              SELECT username, admin
              FROM users
              WHERE username = $1
              "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT username
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_id_by_api_key(
        pool: &PgPool,
        api_key: &str,
    ) -> Result<Option<UserWithKey>, Error> {
        let sha = key_sha(api_key);

        let user = sqlx::query_as!(
            UserWithKey,
            r#"
            SELECT u.username, k.api_key_hash
            FROM users u
            JOIN api_keys k ON u.username = k.user_id
            WHERE k.sha = $1
            "#,
            sha
        )
        .fetch_optional(pool)
        .await?;

        Ok(user.filter(|user| user.verify(api_key)))
    }

    pub async fn get_api_keys(&self, pool: &PgPool) -> Result<Vec<ApiKey>, Error> {
        let api_keys = sqlx::query_as!(
            ApiKey,
            r#"
            SELECT id, created_at, label
            FROM api_keys
            WHERE user_id = $1
            "#,
            self.username
        )
        .fetch_all(pool)
        .await?;

        Ok(api_keys)
    }

    pub async fn delete_api_key(&self, pool: &PgPool, api_key_id: i32) -> Result<bool, Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM api_keys
            WHERE id = $1 AND user_id = $2
            "#,
            api_key_id,
            self.username
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn count(pool: &PgPool) -> Result<i64, Error> {
        let count = sqlx::query_scalar!(r#" SELECT COUNT(*) FROM users"#)
            .fetch_one(pool)
            .await?;
        Ok(count.unwrap_or_default())
    }

    pub async fn all(pool: &PgPool, limit: i64, offset: i64) -> Result<(i64, Vec<User>), Error> {
        let users = sqlx::query_as!(
            UserWithTotal,
            r#" SELECT username , COUNT(*) OVER () as total FROM users ORDER BY username LIMIT $1 OFFSET $2"#,
            limit,
            offset,
        )
        .fetch_all(pool)
        .await?;

        let total = users.first().and_then(|u| u.total).unwrap_or_default();
        let users = users.into_iter().map(|u| u.into()).collect();
        Ok((total, users))
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiKey {
    pub id: i64,
    pub label: String,
    pub created_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize)]
pub struct UserWithKey {
    pub username: String,
    pub api_key_hash: String,
}

impl UserWithKey {
    pub fn verify(&self, api_key: &str) -> bool {
        verify_api_key(api_key, &self.api_key_hash)
    }
}

impl From<UserWithKey> for User {
    fn from(user: UserWithKey) -> Self {
        Self {
            username: user.username,
        }
    }
}
