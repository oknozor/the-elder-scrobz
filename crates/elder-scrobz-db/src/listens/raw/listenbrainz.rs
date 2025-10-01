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
    use crate::listens::raw::listenbrainz::{ListenType, typed};
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
        #[serde(default = "now_timestamp")]
        pub listened_at: i64,
        pub track_metadata: serde_json::Value,
    }

    fn now_timestamp() -> i64 {
        chrono::Utc::now().timestamp()
    }

    impl TryInto<typed::SubmitListens> for SubmitListens {
        type Error = serde_json::Error;

        fn try_into(self) -> Result<typed::SubmitListens, Self::Error> {
            let payload = self
                .payload
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(typed::SubmitListens {
                listen_type: self.listen_type,
                payload,
            })
        }
    }

    impl TryInto<typed::SubmitListensPayload> for SubmitListensPayload {
        type Error = serde_json::Error;

        fn try_into(self) -> Result<typed::SubmitListensPayload, Self::Error> {
            let track_metadata = serde_json::from_value(self.track_metadata)?;
            Ok(typed::SubmitListensPayload {
                listened_at: self.listened_at,
                track_metadata,
            })
        }
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

    impl SubmitListensPayload {
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

        pub fn release_mbid(&self) -> Option<&str> {
            self.additional_info()
                .and_then(|info| info.release_mbid.as_deref())
                .or_else(|| {
                    self.mappings()
                        .and_then(|mapping| mapping.release_mbid.as_deref())
                })
        }

        pub fn track_name(&self) -> &str {
            &self.track_metadata.track_name
        }
        pub fn artist_name(&self) -> &str {
            &self.track_metadata.artist_name
        }

        pub fn release_name(&self) -> &str {
            &self.track_metadata.release_name
        }

        pub fn track_number(&self) -> Option<i32> {
            self.additional_info().and_then(|info| info.tracknumber)
        }

        pub fn track_duration(&self) -> Option<i32> {
            self.additional_info().and_then(|info| info.duration_ms)
        }

        fn mappings(&self) -> Option<&MbidMapping> {
            self.track_metadata.mbid_mapping.as_deref()
        }

        fn additional_info(&self) -> Option<&AdditionalInfo> {
            self.track_metadata.additional_info.as_ref()
        }
    }
}
