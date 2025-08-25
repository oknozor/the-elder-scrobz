-- Add migration script here
ALTER TABLE releases ADD COLUMN subsonic_id TEXT UNIQUE;
ALTER TABLE artists ADD COLUMN subsonic_id TEXT UNIQUE;
ALTER TABLE tracks ADD COLUMN subsonic_id TEXT UNIQUE;
