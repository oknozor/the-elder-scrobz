use crate::PgPool;
pub mod raw;
pub mod scrobble;

#[derive(sqlx::FromRow, Debug)]
pub struct Track {
    pub mbid: String,
    pub release_mbid: String,
    pub name: String,
    pub number: Option<String>,
    pub length: Option<i32>,
}

impl Track {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO tracks (mbid, release_mbid, name, number, length)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (mbid) DO UPDATE
        SET name = COALESCE(EXCLUDED.name, tracks.name),
            number = COALESCE(EXCLUDED.number, tracks.number),
            length = COALESCE(EXCLUDED.length, tracks.length);

        "#,
            self.mbid,
            self.release_mbid,
            self.name,
            self.number,
            self.length,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Release {
    pub mbid: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_art_url: Option<String>,
}

impl Release {
    pub async fn save(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO releases (mbid, name, description, cover_art_url)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (mbid) DO UPDATE
        SET name = COALESCE(EXCLUDED.name, releases.name),
            description = COALESCE(EXCLUDED.description, releases.description),
            cover_art_url = COALESCE(EXCLUDED.cover_art_url, releases.cover_art_url);
        "#,
            self.mbid,
            self.name,
            self.description,
            self.cover_art_url,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Artist {
    pub mbid: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Artist {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO artists (mbid, name, description)
            VALUES ($1, $2, $3)
            ON CONFLICT (mbid) DO UPDATE
            SET name = COALESCE(EXCLUDED.name, artists.name),
                description = COALESCE(EXCLUDED.description, artists.description);
            "#,
            self.mbid,
            self.name,
            self.description,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct ArtistRelease {
    pub artist_mbid: String,
    pub release_mbid: String,
}

impl ArtistRelease {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO artist_releases (artist_mbid, release_mbid)
        VALUES ($1, $2)
        ON CONFLICT (artist_mbid, release_mbid) DO UPDATE
        SET artist_mbid = COALESCE(EXCLUDED.artist_mbid, artist_releases.artist_mbid),
            release_mbid = COALESCE(EXCLUDED.release_mbid, artist_releases.release_mbid);"#,
            self.artist_mbid,
            self.release_mbid,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
