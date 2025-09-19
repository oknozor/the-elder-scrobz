CREATE TABLE scrobbles_errored
(
    id          BIGSERIAL,
    user_id     TEXT                     NOT NULL REFERENCES users (username) ON DELETE CASCADE,
    data        JSONB                    NOT NULL
);
