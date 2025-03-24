CREATE TABLE users
(
    id         TEXT PRIMARY KEY,
    username   TEXT NOT NULL UNIQUE,
    email      TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE api_keys
(
    sha          TEXT PRIMARY KEY,
    user_id      TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    api_key_hash TEXT NOT NULL,
    created_at   TIMESTAMP DEFAULT NOW()
);

CREATE TABLE scrobbles_raw
(
    id         TEXT PRIMARY KEY,
    user_id    TEXT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    data       JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE cover_arts
(
    mbid       TEXT PRIMARY KEY,
    url        TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE releases
(
    mbid           TEXT PRIMARY KEY,
    name           TEXT NOT NULL,
    cover_art_mbid TEXT references cover_arts,
    created_at     TIMESTAMP DEFAULT NOW()

);

CREATE TABLE tracks
(
    mbid         TEXT PRIMARY KEY,
    release_mbid TEXT NOT NULL references releases,
    name         TEXT NOT NULL,
    created_at   TIMESTAMP DEFAULT NOW()
);

CREATE TABLE artists
(
    mbid        TEXT PRIMARY KEY,
    name        TEXT,
    description TEXT,
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE artist_releases
(
    artist_mbid  TEXT NOT NULL references artists,
    release_mbid TEXT NOT NULL references releases,
    created_at   TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (artist_mbid, release_mbid)

);

CREATE TABLE scrobbles
(
    source_id   TEXT NOT NULL REFERENCES scrobbles_raw,
    user_id     TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    track_id    TEXT NOT NULL REFERENCES tracks,
    listened_at TIMESTAMP WITH TIME ZONE,
    created_at  TIMESTAMP DEFAULT NOW()
);


CREATE INDEX idx_user_scrobbles ON scrobbles (user_id);
CREATE INDEX idx_api_keys ON api_keys (user_id);