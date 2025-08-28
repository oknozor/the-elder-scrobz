use chrono::{DateTime, Utc};
use elder_scrobz_db::charts::album::TopAlbum;
use elder_scrobz_db::listens::releases::AlbumDetails as AlbumDetailsEntity;
use elder_scrobz_db::listens::releases::AlbumTrackWithPlayCount as AlbumTrackWithPlayCountEntity;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{SETTINGS, local_image, track::PlayCount};

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

#[derive(Debug, Serialize, ToSchema)]
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
    pub musicbrainz_url: String,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
    pub tracks: Vec<AlbumTrackWithPlayCount>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct AlbumTrackWithPlayCount {
    pub mbid: String,
    pub subsonic_id: Option<String>,
    pub name: String,
    pub number: Option<i32>,
    pub length: Option<i32>,
    pub playcount: Vec<PlayCount>,
    pub total_playcount: i64,
    pub total_listen_duration: Option<i64>,
}

impl From<AlbumTrackWithPlayCountEntity> for AlbumTrackWithPlayCount {
    fn from(track: AlbumTrackWithPlayCountEntity) -> Self {
        let playcount: Vec<PlayCount> = track
            .playcount
            .map(|playcounts| {
                playcounts
                    .iter()
                    .filter_map(|playcount| PlayCount::from_entity(track.length, playcount))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let total_playcount: i64 = playcount.iter().map(|playcount| playcount.count).sum();
        let total_listen_duration = track.length.map(|length| length as i64 * total_playcount);

        AlbumTrackWithPlayCount {
            mbid: track.mbid,
            subsonic_id: track.subsonic_id,
            name: track.name,
            number: track.number,
            length: track.length,
            playcount,
            total_playcount,
            total_listen_duration,
        }
    }
}

impl From<AlbumDetailsEntity> for AlbumDetails {
    fn from(album: AlbumDetailsEntity) -> Self {
        AlbumDetails {
            r#type: "Album",
            thumbnail_url: local_image(&album.id).or(album.thumbnail_url),
            subsonic_url: album.subsonic_id.map(|id| {
                let frontend_url = &SETTINGS.navidrome_frontend_url;
                format!("{frontend_url}/app/#/album/{id}/show")
            }),
            musicbrainz_url: format!("https://musicbrainz.org/release/{}", album.id),
            id: album.id,
            name: album.name,
            artist_id: album.artist_id,
            artist_name: album.artist_name,
            year: album.year,
            description: album.description,
            last_listened_at: album.last_listened_at,
            listens: album.listens,
            tracks: album
                .tracks
                .map(|tracks| {
                    tracks
                        .as_ref() // gives &Vec<TrackWithPlayCount>
                        .iter()
                        .cloned()
                        .map(|t| AlbumTrackWithPlayCount::from(t.clone()))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}
