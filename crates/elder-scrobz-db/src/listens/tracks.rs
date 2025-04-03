use crate::PgPool;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
pub struct Track {
    pub mbid: String,
    pub artist_mbid: String,
    pub release_mbid: String,
    pub artist_display_name: Option<String>,
    pub name: String,
    pub number: Option<i32>,
    pub length: Option<i32>,
}

impl Track {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO tracks (mbid, release_mbid, artist_mbid, artist_display_name, name, number, length)
        VALUES ($1, $2, $3, $4, $5, $6 ,$7)
        ON CONFLICT (mbid) DO UPDATE
        SET name = COALESCE(EXCLUDED.name, tracks.name),
            artist_mbid = COALESCE(EXCLUDED.artist_mbid, tracks.artist_mbid),
            artist_display_name = COALESCE(EXCLUDED.artist_display_name, tracks.artist_display_name),
            number = COALESCE(EXCLUDED.number, tracks.number),
            length = COALESCE(EXCLUDED.length, tracks.length);

        "#,
            self.mbid,
            self.release_mbid,
            self.artist_mbid,
            self.artist_display_name,
            self.name,
            self.number,
            self.length,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn by_id(mbid: &str, db: &PgPool) -> Result<Option<Track>, sqlx::Error> {
        sqlx::query_as!(Track, r#"SELECT mbid, release_mbid, artist_mbid, artist_display_name, name, number, length FROM tracks WHERE mbid = $1"#, mbid)
            .fetch_optional(db)
            .await
    }

    pub async fn by_album(mbid: &str, db: &PgPool) -> Result<Vec<Track>, sqlx::Error> {
        sqlx::query_as!(Track, r#"SELECT mbid, release_mbid, artist_mbid, artist_display_name, name, number, length FROM tracks WHERE release_mbid = $1"#, mbid)
            .fetch_all(db)
            .await
    }
}
