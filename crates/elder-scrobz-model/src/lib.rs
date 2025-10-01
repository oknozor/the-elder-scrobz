use elder_scrobz_settings::Settings;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::get().unwrap());

pub mod album;
pub mod artist;
pub mod events;
pub mod track;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TopListener {
    pub name: String,
    pub playcount: i32,
    pub playtime: i32,
}

fn local_image(mbid: &str) -> Option<String> {
    let image = format!("{mbid}.jpg");
    let coverart_path = SETTINGS.coverart_path.join(image);
    if coverart_path.exists() {
        Some(format!("/coverarts/{mbid}.jpg"))
    } else {
        None
    }
}
