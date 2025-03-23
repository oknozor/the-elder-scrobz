use std::fs;
use std::path::PathBuf;

pub fn scrobble_fixture() -> anyhow::Result<String> {
    let path = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = PathBuf::from(path).join("tests/fixtures/scrobble.json");
    let scrobble = fs::read_to_string(path)?;
    Ok(scrobble)
}
