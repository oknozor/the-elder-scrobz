use crate::PgPool;
use sqlx::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct Scrobble {
    pub source_id: String,
    pub track_id: String,
    pub user_id: String,
}

impl Scrobble {
    pub async fn save(&self, pool: &PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO scrobbles (source_id, track_id, user_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (source_id) DO NOTHING"#,
            self.source_id,
            self.track_id,
            self.user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
