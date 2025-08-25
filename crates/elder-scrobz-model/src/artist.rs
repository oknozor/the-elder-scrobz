use chrono::{DateTime, Utc};
use elder_scrobz_db::charts::artists::TopArtist;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{SETTINGS, local_image};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ChartArtist {
    pub r#type: &'static str,
    pub id: String,
    pub name: Option<String>,
    pub thumbnail_url: Option<String>,
    pub subsonic_url: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
}

impl From<TopArtist> for ChartArtist {
    fn from(artist: TopArtist) -> Self {
        ChartArtist {
            r#type: "Artist",
            thumbnail_url: artist.thumbnail_url.or(local_image(&artist.id)),
            subsonic_url: artist.subsonic_id.map(|id| {
                let frontend_url = &SETTINGS.navidrome_frontend_url;
                format!("{frontend_url}/app/#/artist/{id}/show")
            }),
            id: artist.id,
            name: artist.name,
            last_listened_at: artist.last_listened_at,
            listens: artist.listens,
        }
    }
}
