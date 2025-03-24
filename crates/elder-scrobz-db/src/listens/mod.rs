use crate::PgPool;

#[derive(sqlx::FromRow, Debug)]
pub struct Track {
    pub mbid: String,
    pub release_mbid: String,
    pub name: String,
}

impl Track {
    pub async fn save(
        &self,
        pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO tracks (mbid, release_mbid, name)
        VALUES ($1, $2, $3)
        ON CONFLICT (mbid) DO NOTHING;
        "#,
            self.mbid,
            self.release_mbid,
            self.name,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Release {
    pub mbid: String,
    pub name: Option<String>,
    pub cover_art_mbid: String,
}

impl Release {
    pub async fn save(
        self,
        pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO releases (mbid, name)
        VALUES ($1, $2)
        ON CONFLICT (mbid) DO NOTHING;
        "#,
            self.mbid,
            self.name,
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
            ON CONFLICT (mbid) DO NOTHING;
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
pub struct CoverArt {
    pub mbid: String,
    pub url: Option<String>,
}

impl CoverArt {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO cover_arts (mbid, url)
        VALUES ($1, $2)
        ON CONFLICT (mbid) DO NOTHING;
        "#,
            self.mbid,
            self.url,
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
            r#"
        INSERT INTO artist_releases (artist_mbid, release_mbid)
        VALUES ($1, $2)
        ON CONFLICT (artist_mbid, release_mbid) DO NOTHING;
        "#,
            self.artist_mbid,
            self.release_mbid,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
