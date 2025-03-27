use crate::PgPool;
use crate::listens::raw::listenbrainz::raw;
use chrono::DateTime;
use sqlx::Error;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct CreateRawScrobble {
    pub username: String,
    pub data: raw::SubmitListens,
}

impl CreateRawScrobble {
    pub async fn batch_insert(
        username: String,
        scrobbles: raw::SubmitListens,
        pool: &PgPool,
    ) -> Result<(), Error> {
        for listen in scrobbles.payload {
            let uuid = Uuid::new_v4().to_string();

            let listened_at =
                DateTime::from_timestamp(listen.listened_at, 0).expect("Failed to parse timestamp");

            let raw = serde_json::to_value(&listen).expect("Failed to serialize track metadata");

            sqlx::query!(r#"INSERT INTO scrobbles_raw (user_id, id, data, listened_at) VALUES ($1, $2, $3, $4)
                                ON CONFLICT (user_id, listened_at) DO NOTHING;"#,
                username,
                uuid,
                raw,
                listened_at
            )
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}
