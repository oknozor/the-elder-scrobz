CREATE TYPE scrobble_state AS ENUM ('unprocessed', 'processed');
ALTER TABLE scrobbles_raw ADD COLUMN status scrobble_state NOT NULL DEFAULT 'unprocessed';

