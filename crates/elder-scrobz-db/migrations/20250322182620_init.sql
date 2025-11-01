CREATE TABLE users
(
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE api_keys
(
    sha TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    api_key_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE scrobbles_raw
(
    id TEXT PRIMARY KEY,
    listened_at TIMESTAMP WITH TIME ZONE NOT NULL,
    user_id TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    data JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE (user_id, listened_at)
);

CREATE TABLE artists
(
    mbid TEXT PRIMARY KEY,
    name TEXT,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE releases
(
    mbid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    artist_mbid TEXT REFERENCES artists,
    description TEXT,
    cover_art_url TEXT,
    created_at TIMESTAMP DEFAULT NOW()

);

CREATE TABLE tracks
(
    mbid TEXT PRIMARY KEY,
    artist_mbid TEXT NOT NULL REFERENCES artists,
    release_mbid TEXT NOT NULL REFERENCES releases,
    artist_display_name TEXT,
    name TEXT NOT NULL,
    number INT,
    length INT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE artist_credited
(
    artist_mbid TEXT NOT NULL REFERENCES artists,
    track_mbid TEXT NOT NULL REFERENCES tracks,
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (artist_mbid, track_mbid)

);

CREATE TABLE scrobbles
(
    source_id TEXT PRIMARY KEY REFERENCES scrobbles_raw,
    user_id TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    track_id TEXT NOT NULL REFERENCES tracks,
    created_at TIMESTAMP DEFAULT NOW()
);


CREATE INDEX idx_user_scrobbles ON scrobbles (user_id);
CREATE INDEX idx_api_keys ON api_keys (user_id);
