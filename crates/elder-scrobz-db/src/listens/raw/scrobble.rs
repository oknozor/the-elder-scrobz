use crate::listens::raw::listenbrainz::typed::{self};
use crate::{Pagination, PgPool};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::types::{Json, JsonValue};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, sqlx::Type, Deserialize, Debug)]
pub struct TypedScrobble {
    pub id: String,
    pub user_id: String,
    pub listened_at: DateTime<Utc>,
    pub data: Json<typed::SubmitListensPayload>,
    pub status: ProcessState,
}

#[derive(sqlx::FromRow, sqlx::Type, Deserialize, Serialize, ToSchema, Debug)]
pub struct RawScrobble {
    pub id: String,
    pub user_id: String,
    pub listened_at: DateTime<Utc>,
    pub data: JsonValue,
    pub status: ProcessState,
}

impl TryFrom<RawScrobble> for TypedScrobble {
    type Error = serde_json::Error;

    fn try_from(value: RawScrobble) -> Result<Self, Self::Error> {
        let data = serde_json::from_value::<typed::SubmitListensPayload>(value.data)?;

        Ok(Self {
            id: value.id,
            user_id: value.user_id,
            listened_at: value.listened_at,
            data: Json(data),
            status: value.status,
        })
    }
}

#[derive(Debug, sqlx::Type, Deserialize, Serialize, ToSchema)]
#[sqlx(type_name = "scrobble_state", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProcessState {
    Unprocessed,
    Processed,
}

impl TypedScrobble {
    pub async fn get_by_id(
        pool: &PgPool,
        scrobble_id: &str,
    ) -> Result<Option<TypedScrobble>, Error> {
        sqlx::query_as!(TypedScrobble,
            r#"SELECT id, user_id, data as "data: Json<typed::SubmitListensPayload>", listened_at, status AS "status: ProcessState"
            FROM scrobbles_raw
            WHERE id = $1"#,
            scrobble_id
        )
            .fetch_optional(pool)
            .await
    }

    pub async fn set_processed(pool: &PgPool, scrobble_id: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE scrobbles_raw
            SET status = 'processed'
            WHERE id = $1
        "#,
            scrobble_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl RawScrobble {
    pub async fn get_by_id(pool: &PgPool, scrobble_id: &str) -> Result<Option<RawScrobble>, Error> {
        sqlx::query_as!(
            RawScrobble,
            r#"SELECT id, user_id, data, listened_at, status AS "status: ProcessState"
            FROM scrobbles_raw
            WHERE id = $1"#,
            scrobble_id
        )
        .fetch_optional(pool)
        .await
    }

    // TODO: pagination
    pub async fn get_unprocessed(pool: &PgPool) -> Result<Vec<RawScrobble>, Error> {
        sqlx::query_as(
            r#"
            SELECT id, user_id, data, listened_at, status
            FROM scrobbles_raw
            WHERE status = 'unprocessed'
        "#,
        )
        .fetch_all(pool)
        .await
    }

    // TODO: pagination
    pub async fn all(pool: &PgPool) -> Result<Vec<RawScrobble>, Error> {
        sqlx::query_as(
            r#"
            SELECT id, user_id, data, listened_at, status
            FROM scrobbles_raw
        "#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn by_user_id<P>(
        pool: &PgPool,
        user_id: &str,
        pagination: P,
    ) -> Result<Vec<RawScrobble>, Error>
    where
        P: Into<Pagination>,
    {
        let pagination = pagination.into();
        sqlx::query_as!(
            RawScrobble,
            r#"SELECT id, user_id, data, listened_at, status AS "status: ProcessState"
                FROM scrobbles_raw
                WHERE user_id = $1
                LIMIT $2 OFFSET $3"#,
            user_id,
            pagination.limit,
            pagination.offset
        )
        .fetch_all(pool)
        .await
    }
}

impl TypedScrobble {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn recording_mbid(&self) -> Option<String> {
        self.data.0.recording_mbid()
    }

    pub fn artist_mbids(&self) -> Option<Vec<String>> {
        self.data.0.artist_mbids()
    }

    pub fn release_mbid(&self) -> Option<&str> {
        self.data.0.release_mbid()
    }

    pub fn track_name(&self) -> &str {
        self.data.0.track_name()
    }
    pub fn artist_name(&self) -> &str {
        self.data.0.artist_name()
    }

    pub fn release_name(&self) -> &str {
        self.data.0.release_name()
    }

    pub fn track_number(&self) -> Option<i32> {
        self.data.0.track_number()
    }

    pub fn track_duration(&self) -> Option<i32> {
        self.data.0.track_duration()
    }
}
