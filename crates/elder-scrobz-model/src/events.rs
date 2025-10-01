use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum ScrobzEvent {
    PlayingNow {
        user: String,
        track_name: String,
        artist: String,
        album: String,
        cover_art_url: Option<String>,
        track_duration: i32,
    },
}
