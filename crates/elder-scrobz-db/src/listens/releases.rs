use crate::listens::tracks::Track;
use crate::{PgPool, WithLocalImage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct Release {
    pub mbid: String,
    pub name: String,
    pub artist_mbid: Option<String>,
    pub description: Option<String>,
    pub cover_art_url: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, ToSchema, Debug)]
pub struct AlbumDetails {
    pub id: Option<String>,
    pub name: Option<String>,
    pub release_id: String,
    pub release_name: String,
    pub description: Option<String>,
    pub cover_art_url: Option<String>,
    pub last_listened_at: Option<DateTime<Utc>>,
    pub listens: Option<i64>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct AlbumWithTracks {
    #[serde(flatten)]
    pub album: AlbumDetails,
    pub tracks: Vec<Track>,
}

impl AlbumWithTracks {
    pub async fn by_id(mbid: &str, db: &PgPool) -> Result<AlbumWithTracks, sqlx::Error> {
        let album = sqlx::query_file_as!(AlbumDetails, "queries/details/album.sql", mbid)
            .fetch_one(db)
            .await?;

        let tracks = Track::by_album(mbid, db).await?;

        Ok(AlbumWithTracks { album, tracks })
    }
}

impl Release {
    pub async fn save(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO releases (mbid, name, artist_mbid, description, cover_art_url)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (mbid) DO UPDATE
        SET name = COALESCE(EXCLUDED.name, releases.name),
            description = COALESCE(EXCLUDED.description, releases.description),
            cover_art_url = COALESCE(EXCLUDED.cover_art_url, releases.cover_art_url),
            artist_mbid = COALESCE(EXCLUDED.artist_mbid, releases.artist_mbid);
        "#,
            self.mbid,
            self.name,
            self.artist_mbid,
            self.description,
            self.cover_art_url,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn all_ids(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
        let ids = sqlx::query_scalar!(r#"SELECT mbid FROM releases"#)
            .fetch_all(pool)
            .await?;

        Ok(ids)
    }

    pub async fn missing_coverart(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar!(r#"SELECT mbid FROM releases WHERE cover_art_url IS NULL"#)
            .fetch_all(pool)
            .await
    }

    pub async fn remove_coverart(mbid: &str, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE releases SET cover_art_url = NULL WHERE mbid = $1"#,
            mbid
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl WithLocalImage for Release {
    fn set_image_path(&mut self, path: String) {
        self.cover_art_url = Some(path);
    }

    fn mbid(&self) -> &str {
        &self.mbid
    }
}

impl WithLocalImage for AlbumWithTracks {
    fn set_image_path(&mut self, path: String) {
        self.album.cover_art_url = Some(path);
    }

    fn mbid(&self) -> &str {
        &self.album.release_id
    }
}
