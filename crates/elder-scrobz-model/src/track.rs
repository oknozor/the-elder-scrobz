use elder_scrobz_db::charts::tracks::TopTrack;
use elder_scrobz_db::listens::tracks::Track as TrackEntity;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{SETTINGS, local_image};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ChartTrack {
    pub r#type: &'static str,
    pub id: String,
    pub name: String,
    pub length: Option<i32>,
    pub release_mbid: String,
    pub release_name: String,
    pub thumbnail_url: Option<String>,
    pub subsonic_url: Option<String>,
    pub listens: Option<i64>,
}

impl From<TopTrack> for ChartTrack {
    fn from(track: TopTrack) -> Self {
        ChartTrack {
            r#type: "Track",
            thumbnail_url: local_image(&track.release_mbid).or(track.thumbnail_url),
            id: track.id,
            name: track.name,
            length: track.length,
            release_mbid: track.release_mbid,
            release_name: track.release_name,
            subsonic_url: track.release_subsonic_id.map(|id| {
                let frontend_url = &SETTINGS.navidrome_frontend_url;
                format!("{frontend_url}/app/#/album/{id}/show")
            }),
            listens: track.listens,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Track {
    pub mbid: String,
    pub artist_mbid: String,
    pub release_mbid: String,
    pub subsonic_id: Option<String>,
    pub artist_display_name: Option<String>,
    pub name: String,
    pub number: Option<i32>,
    pub length: Option<i32>,
}

impl From<TrackEntity> for Track {
    fn from(track: TrackEntity) -> Self {
        Track {
            mbid: track.mbid,
            artist_mbid: track.artist_mbid,
            release_mbid: track.release_mbid,
            subsonic_id: track.subsonic_id,
            artist_display_name: track.artist_display_name,
            name: track.name,
            number: track.number,
            length: track.length,
        }
    }
}
