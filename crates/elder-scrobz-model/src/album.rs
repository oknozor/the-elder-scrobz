use chrono::{DateTime, Utc};
use elder_scrobz_db::charts::album::TopAlbum;
use elder_scrobz_db::listens::releases::AlbumWithTracks as AlbumWithTracksEntity;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{SETTINGS, local_image, track::Track};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ChartAlbum {
    pub r#type: &'static str,
    pub id: String,
    pub name: String,
    pub thumbnail_url: Option<String>,
    pub subsonic_url: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
    pub year: Option<i32>,
}

impl From<TopAlbum> for ChartAlbum {
    fn from(album: TopAlbum) -> Self {
        ChartAlbum {
            r#type: "Album",
            thumbnail_url: local_image(&album.id).or(album.thumbnail_url),
            subsonic_url: album.subsonic_id.map(|id| {
                let frontend_url = &SETTINGS.navidrome_frontend_url;
                format!("{frontend_url}/app/#/album/{id}/show")
            }),
            id: album.id,
            name: album.name,
            last_listened_at: album.last_listened_at,
            listens: album.listens,
            year: album.year,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AlbumDetails {
    pub r#type: &'static str,
    pub id: String,
    pub name: String,
    pub artist_id: Option<String>,
    pub artist_name: Option<String>,
    pub year: Option<i32>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub subsonic_url: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
    pub tracks: Vec<Track>,
}

impl From<AlbumWithTracksEntity> for AlbumDetails {
    fn from(album: AlbumWithTracksEntity) -> Self {
        AlbumDetails {
            r#type: "Album",
            thumbnail_url: local_image(&album.album.id).or(album.album.thumbnail_url),
            subsonic_url: album.album.subsonic_id.map(|id| {
                let frontend_url = &SETTINGS.navidrome_frontend_url;
                format!("{frontend_url}/app/#/album/{id}/show")
            }),
            id: album.album.id,
            name: album.album.name,
            artist_id: album.album.artist_id,
            artist_name: album.album.artist_name,
            year: album.album.year,
            description: album.album.description,
            last_listened_at: album.album.last_listened_at,
            listens: album.album.listens,
            tracks: album.tracks.into_iter().map(Track::from).collect(),
        }
    }
}
