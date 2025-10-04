use anyhow::{anyhow, Result};
use clap::Parser;
use musicbrainz_rs::entity::recording::Recording;
use musicbrainz_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "fake-scrobble-gen")]
#[command(about = "Generate fake scrobble JSON from a MusicBrainz recording ID")]
struct Args {
    /// MusicBrainz recording ID (track MBID)
    #[arg(help = "The MusicBrainz recording ID to generate a scrobble for")]
    recording_mbid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FakeScrobble {
    listen_type: String,
    payload: Vec<Payload>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    listened_at: u64,
    track_metadata: TrackMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrackMetadata {
    additional_info: AdditionalInfo,
    artist_name: String,
    release_name: String,
    track_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AdditionalInfo {
    artist_mbids: Vec<String>,
    artist_names: Vec<String>,
    duration_ms: Option<i64>,
    recording_mbid: String,
    release_group_mbid: Option<String>,
    release_mbid: Option<String>,
    submission_client: String,
    submission_client_version: String,
    tracknumber: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Validate the MBID format
    let recording_uuid = Uuid::parse_str(&args.recording_mbid)
        .map_err(|_| anyhow!("Invalid MBID format: {}", args.recording_mbid))?;

    // Fetch recording data from MusicBrainz
    let recording = fetch_recording_data(&recording_uuid.to_string()).await?;

    // Generate fake scrobble
    let scrobble = generate_fake_scrobble(recording)?;

    // Output JSON
    let json_output = serde_json::to_string_pretty(&scrobble)?;
    println!("{}", json_output);

    Ok(())
}

async fn fetch_recording_data(recording_mbid: &str) -> Result<Recording> {
    // Include releases and artists to get release and artist information
    let recording = Recording::fetch()
        .id(recording_mbid)
        .with_releases()
        .with_artists()
        .execute()
        .await
        .map_err(|e| anyhow!("Failed to fetch recording from MusicBrainz: {}", e))?;

    Ok(recording)
}

fn generate_fake_scrobble(recording: Recording) -> Result<FakeScrobble> {
    // Get current timestamp
    let listened_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Extract artist information
    let artist_credits = recording
        .artist_credit
        .ok_or_else(|| anyhow!("No artist credits found for recording"))?;

    let mut artist_mbids = Vec::new();
    let mut artist_names = Vec::new();
    let mut primary_artist_name = String::new();

    for credit in &artist_credits {
        let artist = &credit.artist;
        // Artist ID is a String, not Option<String>
        artist_mbids.push(artist.id.clone());
        artist_names.push(artist.name.clone());
        if primary_artist_name.is_empty() {
            primary_artist_name = artist.name.clone();
        }
    }

    // Get release information from the first available release
    let release = recording
        .releases
        .as_ref()
        .and_then(|releases| releases.first())
        .ok_or_else(|| anyhow!("No releases found for recording"))?;

    let release_name = release.title.clone();
    let release_mbid = Some(release.id.clone());
    let release_group_mbid = release.release_group.as_ref().map(|rg| rg.id.clone());

    // Find track number if available
    let tracknumber = release
        .media
        .as_ref()
        .and_then(|media| media.first())
        .and_then(|medium| medium.tracks.as_ref())
        .and_then(|tracks| {
            tracks
                .iter()
                .find(|track| track.recording.as_ref().map(|rec| &rec.id) == Some(&recording.id))
        })
        .and_then(|track| track.number.parse::<i32>().ok());

    let track_name = recording.title.clone();
    let duration_ms = recording.length.map(|len| len as i64);
    let recording_mbid = recording.id.clone();

    let scrobble = FakeScrobble {
        listen_type: "single".to_string(),
        payload: vec![Payload {
            listened_at,
            track_metadata: TrackMetadata {
                additional_info: AdditionalInfo {
                    artist_mbids,
                    artist_names,
                    duration_ms,
                    recording_mbid,
                    release_group_mbid,
                    release_mbid,
                    submission_client: "fake-scrobble-gen".to_string(),
                    submission_client_version: "0.1.0".to_string(),
                    tracknumber,
                },
                artist_name: primary_artist_name,
                release_name,
                track_name,
            },
        }],
    };

    Ok(scrobble)
}
