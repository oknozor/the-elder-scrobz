use elder_scrobz_settings::Settings;
use once_cell::sync::Lazy;

static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::get().unwrap());

pub mod album;
pub mod artist;
pub mod track;

fn local_image(mbid: &str) -> Option<String> {
    let image = format!("{mbid}.jpg");
    let coverart_path = SETTINGS.coverart_path.join(image);
    if coverart_path.exists() {
        Some(format!("/coverarts/{mbid}.jpg"))
    } else {
        None
    }
}
