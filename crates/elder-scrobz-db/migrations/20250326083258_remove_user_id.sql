DROP INDEX IF EXISTS idx_user_scrobbles;
DROP INDEX IF EXISTS idx_api_keys;

ALTER TABLE api_keys DROP CONSTRAINT IF EXISTS api_keys_user_id_fkey;
ALTER TABLE scrobbles_raw DROP CONSTRAINT IF EXISTS scrobbles_raw_user_id_fkey;
ALTER TABLE scrobbles DROP CONSTRAINT IF EXISTS scrobbles_user_id_fkey;

ALTER TABLE api_keys
    ADD CONSTRAINT api_keys_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(username) ON DELETE CASCADE;

ALTER TABLE scrobbles_raw
    ADD CONSTRAINT scrobbles_raw_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(username) ON DELETE CASCADE;

ALTER TABLE scrobbles
    ADD CONSTRAINT scrobbles_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(username) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_api_keys ON api_keys (user_id);
CREATE INDEX IF NOT EXISTS idx_user_scrobbles ON scrobbles (user_id);


ALTER TABLE users DROP CONSTRAINT IF EXISTS users_pkey;
ALTER TABLE users DROP COLUMN IF EXISTS id;
ALTER TABLE users ALTER COLUMN username SET NOT NULL;
ALTER TABLE users ADD CONSTRAINT users_pkey PRIMARY KEY (username);
