use crate::PgPool;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::types::Json;
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(sqlx::FromRow, sqlx::Type, Debug)]
pub struct CreateRawScrobble {
    pub username: String,
    pub data: Json<Listen>,
}

impl From<SubmitListens> for Vec<Listen> {
    fn from(value: SubmitListens) -> Self {
        value
            .payload
            .into_iter()
            .map(|listen| Listen {
                listen_type: value.listen_type,
                payload: listen,
            })
            .collect()
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Deserialize, Debug)]
pub struct RawScrobble {
    pub id: String,
    pub user_id: String,
    pub listened_at: DateTime<Utc>,
    pub data: Json<Listen>,
    pub status: ProcessState,
}

#[derive(Debug, sqlx::Type, Deserialize)]
#[sqlx(type_name = "scrobble_state", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProcessState {
    Unprocessed,
    Processed,
}

impl RawScrobble {
    pub async fn get_by_id(pool: &PgPool, scrobble_id: &str) -> Result<Option<RawScrobble>, Error> {
        sqlx::query_as!(RawScrobble,
            r#"SELECT id, user_id, data as "data: Json<Listen>", listened_at, status AS "status: ProcessState"
            FROM scrobbles_raw
            WHERE id = $1"#,
            scrobble_id
        )
            .fetch_optional(pool) // Use `fetch_optional` since it may or may not exist
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

    pub async fn get_unprocessed(pool: &PgPool) -> Result<Vec<RawScrobble>, Error> {
        sqlx::query_as(
            r#"
            SELECT id, user_id, data, listened_at, status
            FROM scrobbles_raw
            WHERE status = 'unprocessed'
        "#,
        )
        .fetch_all(pool) // Fetch all rows that match the query
        .await
    }
}

impl CreateRawScrobble {
    pub async fn batch_insert(
        scrobbles: Vec<CreateRawScrobble>,
        pool: &PgPool,
    ) -> Result<(), Error> {
        for scrobble in scrobbles {
            let listened_at = DateTime::from_timestamp(scrobble.data.payload.listened_at, 0)
                .expect("Failed to parse timestamp");
            let uuid = Uuid::new_v4().to_string();
            let value = serde_json::to_value(&scrobble.data).unwrap();
            sqlx::query!(r#"INSERT INTO scrobbles_raw (user_id, id, data, listened_at) VALUES ($1, $2, $3, $4)
                                ON CONFLICT (user_id, listened_at) DO NOTHING;"#,
                scrobble.username,
                uuid,
                value,
                listened_at
            )
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}

impl RawScrobble {
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
        self.data.0.payload.track_metadata.track_name.clone()
    }
    pub fn artist_name(&self) -> String {
        self.data.0.payload.track_metadata.artist_name.clone()
    }

    pub fn release_name(&self) -> String {
        self.data.0.payload.track_metadata.release_name.clone()
    }

    pub fn track_number(&self) -> Option<i32> {
        self.additional_info().and_then(|info| info.tracknumber)
    }

    pub fn track_duration(&self) -> Option<i32> {
        self.additional_info().and_then(|info| info.tracknumber)
    }

    fn mappings(&self) -> Option<&MbidMapping> {
        self.data.0.payload.track_metadata.mbid_mapping.as_deref()
    }

    fn additional_info(&self) -> Option<&AdditionalInfo> {
        self.data.0.payload.track_metadata.additional_info.as_ref()
    }
}

const SUBMIT_LISTEN_DOCS: &str = include_str!("../../docs/submitlisten.example.json");

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[schema(example = json!(SUBMIT_LISTEN_DOCS))]
pub struct SubmitListens {
    pub listen_type: ListenType,
    pub payload: Vec<SubmitListensPayload>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Listen {
    pub listen_type: ListenType,
    pub payload: SubmitListensPayload,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ListenType {
    Single,
    PlayingNow,
    Import,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct SubmitListensPayload {
    pub listened_at: i64,
    pub track_metadata: TrackMetadata,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TrackMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdditionalInfo>,
    pub artist_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mbid_mapping: Option<Box<MbidMapping>>,
    pub release_name: String,
    pub track_name: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct AdditionalInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_mbid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_mbid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracknumber: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_mbids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_player: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_player_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_client_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_msid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_group_mbid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_mbid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_mbids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotify_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discnumber: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listening_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_artist_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_artist_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotify_album_artist_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotify_album_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotify_artist_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albumartist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_names: Option<Vec<String>>,
    #[serde(alias = "trackNumber", skip_serializing_if = "Option::is_none")]
    pub track_number: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct MbidMapping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_mbids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<TopReleasesForUserPayloadReleasesInnerArtistsInner>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caa_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caa_release_mbid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_mbid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_mbid: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TopReleasesForUserPayloadReleasesInnerArtistsInner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_credit_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_mbid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_phrase: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::listens::raw::ListenType::Single;
    use crate::listens::raw::{AdditionalInfo, SubmitListens, SubmitListensPayload, TrackMetadata};

    #[test]
    fn deserialize_submit_listens() {
        let listens = SubmitListens {
            listen_type: Single,
            payload: Vec::from([SubmitListensPayload {
                listened_at: 1742999763,
                track_metadata: TrackMetadata {
                    additional_info: Some(AdditionalInfo {
                        recording_mbid: Some("9f3748d6-b4db-41d7-875e-43d8e273575c".to_string()),
                        release_mbid: Some("ee3a8b51-b67d-41a5-bbc6-2cb427677947".to_string()),
                        duration_ms: Some(235930),
                        tracknumber: Some(13),
                        artist_mbids: Some(vec!["93dda03d-c1d3-42c2-80b0-fa71bc20262b".to_string()]),
                        media_player: None,
                        media_player_version: None,
                        submission_client: Some("navidrome".to_string()),
                        submission_client_version: Some("0.55.1 (beb768cd)".to_string()),
                        music_service: None,
                        music_service_name: None,
                        origin_url: None,
                        recording_msid: None,
                        tags: None,
                        duration: None,
                        release_group_mbid: Some("2d1a315e-ae5a-3648-ab58-59a45ce5bac5".to_string()),
                        track_mbid: None,
                        work_mbids: None,
                        isrc: None,
                        spotify_id: None,
                        discnumber: None,
                        listening_from: None,
                        release_artist_name: None,
                        release_artist_names: None,
                        spotify_album_artist_ids: None,
                        spotify_album_id: None,
                        spotify_artist_ids: None,
                        youtube_id: None,
                        albumartist: None,
                        comment: None,
                        date: None,
                        genre: None,
                        artist_names: Some(vec!["Capleton".to_string()]),
                        track_number: None,
                    }),
                    artist_name: "Capleton".to_string(),
                    mbid_mapping: None,
                    release_name: "I Testament".to_string(),
                    track_name: "Death Row".to_string(),
                },
            }]),
        };
        let json = serde_json::to_string_pretty(&listens).unwrap();
        println!("{}", json);

    }
}
