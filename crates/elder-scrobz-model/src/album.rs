use chrono::{DateTime, Utc};
use elder_scrobz_db::charts::album::TopAlbum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{SETTINGS, local_image};

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
            thumbnail_url: album.thumbnail_url.or(local_image(&album.id)),
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
