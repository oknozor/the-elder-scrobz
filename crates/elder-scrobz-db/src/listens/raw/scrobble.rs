use crate::PgPool;
use crate::listens::raw::listenbrainz::typed;
use crate::listens::raw::listenbrainz::typed::{AdditionalInfo, MbidMapping};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::types::{Json, JsonValue};

#[derive(sqlx::FromRow, sqlx::Type, Deserialize, Debug)]
pub struct TypedScrobble {
    pub id: String,
    pub user_id: String,
    pub listened_at: DateTime<Utc>,
    pub data: Json<typed::SubmitListensPayload>,
    pub status: ProcessState,
}

#[derive(sqlx::FromRow, sqlx::Type, Deserialize, Serialize, Debug)]
pub struct RawScrobble {
    pub id: String,
    pub user_id: String,
    pub listened_at: DateTime<Utc>,
    pub data: JsonValue,
    pub status: ProcessState,
}

#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
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

    // TODO: pagination
    pub async fn all(pool: &PgPool) -> Result<Vec<TypedScrobble>, Error> {
        sqlx::query_as(
            r#"
            SELECT id, user_id, data, listened_at, status
            FROM scrobbles_raw
        "#,
        )
        .fetch_all(pool)
        .await
    }

    // TODO: pagination
    pub async fn get_unprocessed(pool: &PgPool) -> Result<Vec<TypedScrobble>, Error> {
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

#[test]
fn test() {
    let t = r#"{"id":"2fec630b-b2ce-4c5e-8a5a-eb7df1784998","listened_at":"2025-01-11T09:34:51+00:00","user_id":"oknozor","data":{"listened_at": 1736588091, "track_metadata": {"track_name": "m.A.A.d city", "artist_name": "Kendrick Lamar feat. MC Eiht", "mbid_mapping": {"caa_id": 22057643526, "artists": [{"artist_mbid": "381086ea-f511-4aba-bdf9-71c753dc5077", "join_phrase": " featuring ", "artist_credit_name": "Kendrick Lamar"}, {"artist_mbid": "103555b8-ae47-4e99-8fc3-e31bdfd7854e", "join_phrase": "", "artist_credit_name": "MC Eiht"}], "artist_mbids": ["381086ea-f511-4aba-bdf9-71c753dc5077", "103555b8-ae47-4e99-8fc3-e31bdfd7854e"], "release_mbid": "e1d99364-1ad9-4f4d-9505-2242eff10a44", "recording_mbid": "d142987d-69a5-426c-8d41-20ce2cc1534b", "recording_name": "m.A.A.d city", "caa_release_mbid": "d5bcadc9-d6b2-4119-bf50-af1b9dca834c"}, "release_name": "good kid, m.A.A.d city", "recording_msid": "1211efde-8222-4193-a569-078586594333", "additional_info": {"duration_ms": 350120, "tracknumber": 8, "artist_mbids": ["381086ea-f511-4aba-bdf9-71c753dc5077"], "release_mbid": "bb772ff7-7ed8-435b-9bfe-90df819fa605", "recording_mbid": "d142987d-69a5-426c-8d41-20ce2cc1534b", "submission_client": "navidrome", "submission_client_version": "0.54.3 (734eb30a)"}}},"created_at":"2025-03-27T13:48:32.221156","status":"unprocessed"}"#;
    let t: TypedScrobble = serde_json::from_str(t).unwrap();
}
