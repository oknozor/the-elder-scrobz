use crate::{PgPool, WithLocalImage};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema, Debug)]
pub struct Artist {
    pub mbid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
}

impl Artist {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO artists (mbid, name, description, thumbnail_url)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (mbid) DO UPDATE
            SET name = COALESCE(EXCLUDED.name, artists.name),
                description = COALESCE(EXCLUDED.description, artists.description),
                thumbnail_url = COALESCE(EXCLUDED.thumbnail_url, artists.thumbnail_url);
            "#,
            self.mbid,
            self.name,
            self.description,
            self.thumbnail_url,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn by_id(mbid: &str, pool: &PgPool) -> Result<Artist, sqlx::Error> {
        sqlx::query_as!(
            Artist,
            r#"SELECT mbid, name, description, thumbnail_url FROM artists WHERE mbid = $1;"#,
            mbid
        )
        .fetch_one(pool)
        .await
    }
    pub async fn all_ids(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
        let ids = sqlx::query_scalar!(r#"SELECT mbid FROM artists"#)
            .fetch_all(pool)
            .await?;

        Ok(ids)
    }

    pub async fn with_missing_metadata(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar!(
            r#"SELECT mbid FROM artists WHERE thumbnail_url IS NULL OR description IS NULL"#
        )
        .fetch_all(pool)
        .await
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct ArtistCredited {
    pub artist_mbid: String,
    pub track_mbid: String,
}

impl ArtistCredited {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO artist_credited (artist_mbid, track_mbid)
        VALUES ($1, $2)
        ON CONFLICT (artist_mbid, track_mbid) DO UPDATE
        SET artist_mbid = COALESCE(EXCLUDED.artist_mbid, artist_credited.artist_mbid),
            track_mbid = COALESCE(EXCLUDED.track_mbid, artist_credited.track_mbid);"#,
            self.artist_mbid,
            self.track_mbid,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl WithLocalImage for Artist {
    fn mbid(&self) -> &str {
        &self.mbid
    }

    fn set_image_path(&mut self, path: String) {
        self.thumbnail_url = Some(path);
    }
}
