use serde::Deserialize;
use sqlx::types::Json;

use crate::PgPool;

#[derive(sqlx::FromRow, Debug)]
pub struct Track {
    pub mbid: String,
    pub artist_mbid: String,
    pub release_mbid: String,
    pub subsonic_id: Option<String>,
    pub artist_display_name: Option<String>,
    pub name: String,
    pub number: Option<i32>,
    pub length: Option<i32>,
}

#[derive(sqlx::FromRow, Deserialize, Debug, Clone)]
pub struct TrackWithPlayCount {
    #[serde(default)]
    pub mbid: String,
    pub artist_mbid: String,
    pub release_mbid: String,
    pub subsonic_id: Option<String>,
    pub artist_display_name: Option<String>,
    pub name: String,
    pub number: Option<i32>,
    pub length: Option<i32>,
    pub playcount: Option<Json<Vec<PlayCount>>>,
}

#[derive(sqlx::FromRow, Deserialize, Debug, Clone)]
pub struct PlayCount {
    pub username: String,
    pub count: Option<i64>,
}

impl TrackWithPlayCount {
    pub async fn by_id(mbid: &str, db: &PgPool) -> Result<Option<TrackWithPlayCount>, sqlx::Error> {
        sqlx::query_as!(
            TrackWithPlayCount,
            r#"SELECT
                t.mbid,
                t.subsonic_id,
                t.release_mbid,
                t.artist_mbid,
                t.artist_display_name,
                t.name,
                t.number,
                t.length,
                COALESCE(
                    (
                        SELECT json_agg(
                                   json_build_object(
                                       'username', pc.user_id,
                                       'count', pc.playcount
                                   )
                               )
                        FROM (
                            SELECT s.user_id, COUNT(*)::bigint AS playcount
                            FROM scrobbles s
                            WHERE s.track_id = t.mbid
                            GROUP BY s.user_id
                            ORDER BY COUNT(*) DESC
                            LIMIT 10
                        ) pc
                    ),
                    '[]'::json
                ) AS "playcount: Json<Vec<PlayCount>>"
            FROM tracks t
            WHERE t.mbid = $1;
            "#,
            mbid
        )
        .fetch_optional(db)
        .await
    }
}

impl Track {
    pub async fn save(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO tracks (mbid, subsonic_id, release_mbid, artist_mbid, artist_display_name, name, number, length)
        VALUES ($1, $2, $3, $4, $5, $6 ,$7, $8)
        ON CONFLICT (mbid) DO UPDATE
        SET name = COALESCE(EXCLUDED.name, tracks.name),
            artist_mbid = COALESCE(EXCLUDED.artist_mbid, tracks.artist_mbid),
            subsonic_id = COALESCE(EXCLUDED.subsonic_id, tracks.subsonic_id),
            artist_display_name = COALESCE(EXCLUDED.artist_display_name, tracks.artist_display_name),
            number = COALESCE(EXCLUDED.number, tracks.number),
            length = COALESCE(EXCLUDED.length, tracks.length);

        "#,
            self.mbid,
            self.subsonic_id,
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
        sqlx::query_as!(Track, r#"SELECT mbid, subsonic_id, release_mbid, artist_mbid, artist_display_name, name, number, length FROM tracks WHERE mbid = $1"#, mbid)
            .fetch_optional(db)
            .await
    }

    pub async fn by_album(mbid: &str, db: &PgPool) -> Result<Vec<Track>, sqlx::Error> {
        sqlx::query_as!(Track, r#"SELECT mbid, subsonic_id, release_mbid, artist_mbid, artist_display_name, name, number, length FROM tracks WHERE release_mbid = $1"#, mbid)
            .fetch_all(db)
            .await
    }
}
