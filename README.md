# The Elder Scrobz

## 🎵 About

The Elder Scrobz is a music tracking application that helps you monitor and analyze your listening habits. Inspired by services like Last.fm, Maloja & Listen Brainz it allows you to "scrobble" (track) the music you listen to and provides detailed statistics and insights about your music taste.

**This is a work in progress and not usable at the moment !**

## ✨ Features

- **Music Statistics Dashboard**: View comprehensive statistics about your listening habits
- **Artist & Album Details**: Explore detailed information about artists and albums you've listened to
- **Track Insights**: Get insights about individual tracks in your listening history
- **Data Import**: Import your listening history from other services (Maloja, Last.FM, ListenBrainz)
- **Multi-User Support**: Manage multiple user profiles
- **MusicBrainz Integration**: Enriched metadata from the MusicBrainz database

## 🛠️ Technologies

- **Backend**: Rust with Axum web framework
- **Frontend**: Vue.js with TypeScript
- **Database**: PostgreSQL
- **API Documentation**: Swagger UI via Utoipa

## Running

The app is fully dockerized, you need to configure the following env vars :

- `SCROBZ__database_url`: A PostgreSQL connection URL (e.g., `postgres://username:password@localhost:5432/scrobz`)
- `SCROBZ__port`: The port on which the backend server will run (defaults to 3000)

In your music app, configure the Listen Brainz scrobble endpoint to the following : `https://the-elder-scrobz.domain.example/api/v1`

## 🚀 Development

- Install sqlx-cli: `cargo install sqlx-cli`
- Start PostgreSQL: `docker compose up -d`
- Create the database: `slqx database create`
- Start the backend: `cargo run`
- Start the frontend: `cd frontend && npm run dev`
