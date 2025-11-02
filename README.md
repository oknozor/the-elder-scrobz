# The Elder Scrobz

## 🎵 About

The Elder Scrobz is a music tracking application that helps you monitor and analyze your listening habits. Inspired by
services like Last.fm, Maloja & Listen Brainz it allows you to "scrobble" (track) the music you listen to and provides
detailed statistics and insights about your music taste.

**This is a work in progress and not usable at the moment !**

## ✨ Features

- **Music Statistics Dashboard**: View comprehensive statistics about you and your friends' listening habits
- **Now Playing**: View in real time what your friends are listening
- **Artist & Album Details**: Explore detailed information about artists and albums you've listened to
- **Data Import**: Import your listening history from other services (Maloja, Last.FM, ListenBrainz)
- **Multi-User Support**: Manage multiple user profiles
- **MusicBrainz & Discogs Integration**: Enriched metadata from MusicBrainz & Discogs

![](./assets/screenshot.png)

## 🛠️ Technologies

- **Backend**: Rust with Axum web framework
- **Frontend**: Vue.js with TypeScript
- **Database**: PostgreSQL
- **API Documentation**: Swagger UI via Utoipa

## Running

The app is fully dockerized, you need to configure the following env vars :


| Variable                         | Description                                             | Example/Notes                                                    |
|----------------------------------|---------------------------------------------------------|------------------------------------------------------------------|
| `SCROBZ__debug`                  | Enable debug mode                                      | `false`                                                           |
| `SCROBZ__port`                   | Port number for the application server                  | `3001`                                                           |
| `SCROBZ__coverart_path`          | Directory path for storing album cover art cache        | `coverarts`                                                      |
| `SCROBZ__database_url`           | PostgreSQL database connection URL                      | `postgres://postgres:postgres@localhost/scrobz`                  |
| `SCROBZ__discogs__key`           | Discogs API consumer key                                |                                                                  |
| `SCROBZ__discogs__secret`        | Discogs API consumer secret                             |                                                                  |
| `SCROBZ__navidrome__server_url`  | Backend API URL for Navidrome music server              | `https://my-navidrome-server.org`                                |
| `SCROBZ__navidrome__frontend_url`| Frontend URL for Navidrome web interface                | `https://my-navidrome-front.org`                                 |
| `SCROBZ__navidrome__username`    | Username for Navidrome authentication                   | `your_username`                                                  |
| `SCROBZ__navidrome__password`    | Password for Navidrome authentication                   | `your_password`                                                  |
| `SCROBZ__oidc__client_id`        | OAuth2/OpenID Connect client ID                         | `your_client_id`                                                 |
| `SCROBZ__oidc__client_secret`    | OAuth2/OpenID Connect client secret                     | `your_client_secret`                                             |
| `SCROBZ__oidc__provider_url`     | OAuth2/OpenID Connect provider authority URL            | `https://sso.hoohoot.org/application/o/the-elder-scrobz`         |
| `SCROBZ__oidc__force_http`       | Force HTTP for OAuth2/OpenID Connect                    | `true`                                                           |
| `SCROBZ__oidc__domain`           | Domain for OAuth2/OpenID Connect                        | `my-domain.org`                                                  |

In your music app, configure the Listen Brainz scrobble endpoint to the following :
`https://the-elder-scrobz.domain.example/api/v1`

## 🚀 Development

- (Optional) Install commit hooks:
    - `cargo install cocogitto`
    - `cog install-hook --all`
- Install sqlx-cli: `cargo install sqlx-cli`
- Start PostgreSQL: `docker compose up -d`
- Create the database: `slqx database create`
- Start the backend: `cargo run`
- Start the frontend: `cd frontend && npm run dev`
