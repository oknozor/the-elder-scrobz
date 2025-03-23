use crate::PgPool;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::types::{Json, Uuid};

#[derive(sqlx::FromRow, Debug)]
struct Scrobble {
    data: Json<Listen>,
}

pub async fn fetch_scrobbles_by_user_and_mbid(
    pool: &PgPool,
    user_id: &str,
    recording_mbid: &str,
) -> Result<Vec<Listen>, Error> {
    let rows = sqlx::query!(
        r#"
        SELECT data
        FROM scrobbles
        WHERE user_id = $1
          AND data->'payload'->'track_metadata'->'mbid_mapping'->>'recording_mbid' = $2;
        "#,
        user_id,
        recording_mbid,
    )
    .fetch_all(pool)
    .await?;

    let listens = rows
        .into_iter()
        .filter_map(|row| {
            serde_json::from_value::<Listen>(row.data).ok() // Manually handle JSON deserialization
        })
        .collect::<Vec<Listen>>();

    Ok(listens)
}

#[derive(sqlx::FromRow, sqlx::Type, Debug)]
pub struct CreateScrobble {
    pub user_id: String,
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

impl CreateScrobble {
    pub async fn batch_insert(
        scrobbles: Vec<CreateScrobble>,
        pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for scrobble in scrobbles {
            sqlx::query("INSERT INTO scrobbles (user_id, id, data) VALUES ($1, $2, $3)")
                .bind(scrobble.user_id)
                .bind(Uuid::new_v4().to_string())
                .bind(&scrobble.data)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitListens {
    pub listen_type: ListenType,
    pub payload: Vec<SubmitListensPayload>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Listen {
    pub listen_type: ListenType,
    pub payload: SubmitListensPayload,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ListenType {
    Single,
    PlayingNow,
    Import,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitListensPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listened_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_metadata: Option<Box<TrackMetadata>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdditionalInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mbid_mapping: Option<Box<MbidMapping>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdditionalInfo {
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
    pub release_mbid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_mbids: Option<Vec<uuid::Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_mbid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_msid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracknumber: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_group_mbid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_mbid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_mbids: Option<Vec<uuid::Uuid>>,
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
    #[serde(rename = "trackNumber", skip_serializing_if = "Option::is_none")]
    pub track_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MbidMapping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_mbids: Option<Vec<uuid::Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<TopReleasesForUserPayloadReleasesInnerArtistsInner>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caa_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caa_release_mbid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_mbid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_mbid: Option<uuid::Uuid>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopReleasesForUserPayloadReleasesInnerArtistsInner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_credit_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_mbid: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_phrase: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::scrobble::SubmitListensPayload;

    #[test]
    fn deserialize_submit_listens() {
        let json = r#"{"inserted_at":1736502941.265404,"listened_at":1736502941,"track_metadata":{"additional_info":{"artist_mbids":["133c77d4-f8a8-4d26-92e2-6729a9264750"],"duration_ms":213990,"recording_mbid":"1b3a5bf1-1f67-4339-be91-77d07d2c9d65","release_mbid":"7a82e8bd-a384-49e6-8aa8-d68b8194e535","submission_client":"navidrome","submission_client_version":"0.54.3 (734eb30a)","tracknumber":1},"artist_name":"Sonic Area","mbid_mapping":{"artist_mbids":["133c77d4-f8a8-4d26-92e2-6729a9264750"],"artists":[{"artist_credit_name":"Sonic Area","artist_mbid":"133c77d4-f8a8-4d26-92e2-6729a9264750","join_phrase":""}],"caa_id":28429645674,"caa_release_mbid":"7a82e8bd-a384-49e6-8aa8-d68b8194e535","recording_mbid":"1b3a5bf1-1f67-4339-be91-77d07d2c9d65","recording_name":"Soot Spirit","release_mbid":"7a82e8bd-a384-49e6-8aa8-d68b8194e535"},"recording_msid":"c22b8ba2-3860-4126-ab32-4e6a8b4fcc41","release_name":"Ki","track_name":"Soot Spirit"}}"#;

        serde_json::from_str::<SubmitListensPayload>(json).unwrap();
    }
}
