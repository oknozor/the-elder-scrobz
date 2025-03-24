use crate::PgPool;
use crate::api_key::{key_sha, verify_api_key};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::types::Uuid;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

impl CreateUser {
    pub async fn insert(self, pool: &PgPool) -> Result<Uuid, Error> {
        let user_id = Uuid::new_v4();

        sqlx::query!(
            r#"
        INSERT INTO users (id, username, email)
        VALUES ($1, $2, $3)
        "#,
            user_id.to_string(),
            self.username,
            self.email,
        )
        .execute(pool)
        .await?;

        Ok(user_id)
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl User {
    pub async fn get_by_id(pool: &PgPool, user_id: &str) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_id_by_api_key(
        pool: &PgPool,
        api_key: &str,
    ) -> Result<Option<String>, Error> {
        let sha = key_sha(api_key);

        let user = sqlx::query_as!(
            UserWithKey,
            r#"
            SELECT u.id, u.username, u.email, k.api_key_hash
            FROM users u
            JOIN api_keys k ON u.id = k.user_id
            WHERE k.sha = $1
            "#,
            sha
        )
        .fetch_optional(pool)
        .await?;

        Ok(user.filter(|user| user.verify(api_key)).map(|u| u.id))
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize)]
pub struct UserWithKey {
    pub id: String,
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
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}
