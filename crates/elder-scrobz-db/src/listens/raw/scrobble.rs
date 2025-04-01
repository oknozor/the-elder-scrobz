use crate::PgPool;
use crate::listens::raw::listenbrainz::typed;
use crate::listens::raw::listenbrainz::typed::{AdditionalInfo, MbidMapping};
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
}

impl TypedScrobble {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn recording_mbid(&self) -> Option<String> {
        self.additional_info()
            .and_then(|info| info.recording_mbid.clone())
            .or_else(|| {
                self.mappings()
                    .and_then(|mapping| mapping.recording_mbid.clone())
            })
    }

    pub fn artist_mbids(&self) -> Option<Vec<String>> {
        self.additional_info()
            .and_then(|info| info.artist_mbids.clone())
            .or_else(|| {
                self.mappings()
                    .and_then(|mapping| mapping.artist_mbids.clone())
            })
    }

    pub fn release_mbid(&self) -> Option<String> {
        self.additional_info()
            .and_then(|info| info.release_mbid.clone())
            .or_else(|| {
                self.mappings()
                    .and_then(|mapping| mapping.release_mbid.clone())
            })
    }

    pub fn track_name(&self) -> String {
        self.data.0.track_metadata.track_name.clone()
    }
    pub fn artist_name(&self) -> String {
        self.data.0.track_metadata.artist_name.clone()
    }

    pub fn release_name(&self) -> String {
        self.data.0.track_metadata.release_name.clone()
    }

    pub fn track_number(&self) -> Option<i32> {
        self.additional_info().and_then(|info| info.tracknumber)
    }

    pub fn track_duration(&self) -> Option<i32> {
        self.additional_info().and_then(|info| info.duration_ms)
    }

    fn mappings(&self) -> Option<&MbidMapping> {
        self.data.0.track_metadata.mbid_mapping.as_deref()
    }

    fn additional_info(&self) -> Option<&AdditionalInfo> {
        self.data.0.track_metadata.additional_info.as_ref()
    }
}
