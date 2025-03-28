use crate::PgPool;
use crate::api_key::{key_sha, verify_api_key};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

impl CreateUser {
    pub async fn insert(self, pool: &PgPool) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2) returning username, email
        "#,
            self.username,
            self.email,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub username: String,
    pub email: String,
}

impl User {
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT username, email
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
            SELECT u.username, u.email, k.api_key_hash
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

    pub async fn all(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Self>, Error> {
        let user = sqlx::query_as!(
            User,
            r#" SELECT username, email FROM users ORDER BY username OFFSET $2 LIMIT $1 "#,
            limit,
            offset,
        )
        .fetch_all(pool)
        .await?;

        Ok(user)
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize)]
pub struct UserWithKey {
    pub username: String,
    pub email: String,
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
            email: user.email,
        }
    }
}
