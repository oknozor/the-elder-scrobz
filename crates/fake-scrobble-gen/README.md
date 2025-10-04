# Fake Scrobble Generator

A Rust binary tool that generates fake scrobble JSON data from a MusicBrainz recording ID (track MBID). This tool fetches track metadata from MusicBrainz and formats it into a ListenBrainz-compatible scrobble format.

## Features

- Fetches track metadata from MusicBrainz API
- Resolves artist MBIDs and names
- Includes release information (release MBID, release group MBID)
- Generates proper ListenBrainz scrobble format
- Includes track number when available
- Uses current timestamp as listen time

## Usage

### Building

From the project root:
```bash
cargo build --bin fake-scrobble-gen
```

### Running

```bash
cargo run --bin fake-scrobble-gen -- <RECORDING_MBID>
```

Or after building:
```bash
./target/debug/fake-scrobble-gen <RECORDING_MBID>
```

### Example

```bash
cargo run --bin fake-scrobble-gen -- bef57649-20d5-47c0-bcaa-10b8dd74ea29
```

This will output JSON like:
```json
{
  "listen_type": "single",
  "payload": [
    {
      "listened_at": 1743194922,
      "track_metadata": {
        "additional_info": {
          "artist_mbids": [
            "071ab0f1-ac3f-48f6-9ee6-11b72eb73e57"
          ],
          "artist_names": [
            "Murray Head"
          ],
          "duration_ms": 259760,
          "recording_mbid": "bef57649-20d5-47c0-bcaa-10b8dd74ea29",
          "release_group_mbid": "7efd3a84-9478-3167-ae07-3ab041e2188f",
          "release_mbid": "83c821d0-13b1-44a6-abc2-00815f62889f",
          "submission_client": "fake-scrobble-gen",
          "submission_client_version": "0.1.0",
          "tracknumber": 3
        },
        "artist_name": "Murray Head",
        "release_name": "Say It Ain't So",
        "track_name": "Boats Away"
      }
    }
  ]
}
```

## Requirements

- Internet connection (to query MusicBrainz API)
- Valid MusicBrainz recording ID (MBID)

## Error Handling

The tool will return appropriate error messages for:
- Invalid MBID format
- Recording not found in MusicBrainz
- Missing required metadata (artist credits, releases)
- Network connectivity issues

## Dependencies

- `musicbrainz_rs` - MusicBrainz API client
- `serde` - JSON serialization
- `chrono` - Date/time handling
- `clap` - Command line argument parsing
- `anyhow` - Error handling
- `uuid` - MBID validation
- `tokio` - Async runtime