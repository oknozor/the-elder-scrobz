use chrono::{DateTime, Utc};
use elder_scrobz_db::charts::album::TopAlbum;

pub struct ChartAlbum {
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
            id: album.id,
            name: album.name,
            thumbnail_url: album.thumbnail_url,
            subsonic_url: album.subsonic_id,
            last_listened_at: album.last_listened_at,
            listens: album.listens,
            year: album.year,
        }
    }
}
