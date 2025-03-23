-- Users table
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

CREATE TABLE scrobbles
(
    id         TEXT PRIMARY KEY,
    user_id    TEXT  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    data       JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_user_scrobbles ON scrobbles (user_id);
CREATE INDEX idx_api_keys ON api_keys (user_id);