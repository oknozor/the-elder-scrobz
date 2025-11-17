CREATE TABLE user_configs (
    username TEXT NOT NULL REFERENCES users (username) UNIQUE,
    enable_weekly_playlist BOOLEAN NOT NULL DEFAULT FALSE,
    enable_monthly_playlist BOOLEAN NOT NULL DEFAULT FALSE,
    enable_yearly_playlist BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE global_config (
    id SERIAL PRIMARY KEY,
    enable_weekly_playlist BOOLEAN NOT NULL DEFAULT FALSE,
    enable_monthly_playlist BOOLEAN NOT NULL DEFAULT FALSE,
    enable_yearly_playlist BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO user_configs (username) SELECT username FROM users;
INSERT INTO global_config (
    enable_weekly_playlist, enable_monthly_playlist, enable_yearly_playlist
)
VALUES (FALSE, FALSE, FALSE);
