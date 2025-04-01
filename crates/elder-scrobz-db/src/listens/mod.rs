use serde::Deserialize;

use crate::PgPool;
pub mod raw;
pub mod recent;
pub mod scrobble;

#[derive(sqlx::FromRow, Debug)]
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
}

#[derive(sqlx::FromRow, Deserialize, Debug)]
pub struct Release {
    pub mbid: String,
    pub name: String,
    pub artist_mbid: Option<String>,
    pub description: Option<String>,
    pub cover_art_url: Option<String>,
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

    pub async fn release_exists_with_cover_art(
        mbid: &str,
        pool: &PgPool,
    ) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar!(
            r#"
        SELECT EXISTS(
            SELECT 1
            FROM releases
            WHERE mbid = $1
              AND cover_art_url IS NOT NULL
        )
        "#,
            mbid
        )
        .fetch_one(pool)
        .await?;

        Ok(exists.unwrap_or_default())
    }

    pub async fn missing_coverart(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar!(r#"SELECT mbid FROM releases WHERE cover_art_url IS NULL"#)
            .fetch_all(pool)
            .await
    }
}

#[derive(sqlx::FromRow, Deserialize, Debug)]
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
