use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const SUBMIT_LISTEN_DOCS: &str = include_str!("../../../docs/submitlisten.example.json");

#[derive(Serialize, Deserialize, ToSchema, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ListenType {
    Single,
    PlayingNow,
    Import,
}

/// These are untyped scrobbled we receive from client,
/// This is needed to keep scrobbles we are not able to process yet.
pub mod raw {
    use crate::listens::raw::listenbrainz::ListenType;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, ToSchema, Debug)]
    #[schema(example = json!(super::SUBMIT_LISTEN_DOCS))]
    pub struct SubmitListens {
        pub listen_type: ListenType,
        pub payload: Vec<SubmitListensPayload>,
    }

    #[derive(Serialize, Deserialize, ToSchema, Debug)]
    pub struct SubmitListensPayload {
        pub listened_at: i64,
        pub track_metadata: serde_json::Value,
    }
}

/// Strongly typed scrobbles for later processing
pub mod typed {
    use crate::listens::raw::listenbrainz::ListenType;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, ToSchema, Debug)]
    #[schema(example = json!(super::SUBMIT_LISTEN_DOCS))]
    pub struct SubmitListens {
        pub listen_type: ListenType,
        pub payload: Vec<SubmitListensPayload>,
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
}
