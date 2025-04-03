use serde::Serialize;
use sqlx::{PgPool, query_scalar};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Stats {
    total_raw_scrobble_count: i64,
    total_scrobble_count: i64,
    total_track_count: i64,
    total_releases_count: i64,
    total_artists_count: i64,
    unparsable_scrobbles: MissingDataStats,
    release_without_coverart: MissingDataStats,
    artist_without_thumbnail: MissingDataStats,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MissingDataStats {
    count: usize,
    ids: Vec<String>,
}

impl Stats {
    pub async fn get(pool: &PgPool) -> Result<Stats, sqlx::Error> {
        let total_raw_scrobble_count = query_scalar!(r#"SELECT count(*) FROM scrobbles_raw"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        let total_scrobble_count = query_scalar!(r#"SELECT count(*) FROM scrobbles"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        let total_releases_count = query_scalar!(r#"SELECT count(*) FROM releases"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        let total_artists_count = query_scalar!(r#"SELECT count(*) FROM artists"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        let unparsable_scrobbles = query_scalar!(
            r#"SELECT r.id FROM scrobbles_raw r
LEFT JOIN scrobbles s ON s.source_id = r.id
WHERE s.source_id IS NULL;"#
        )
        .fetch_all(pool)
        .await?;

        let unparsable_scrobbles = MissingDataStats {
            count: unparsable_scrobbles.len(),
            ids: unparsable_scrobbles,
        };

        let missing_coverarts =
            query_scalar!(r#"SELECT mbid FROM releases WHERE cover_art_url IS NULL"#)
                .fetch_all(pool)
                .await?;

        let release_without_coverart = MissingDataStats {
            count: missing_coverarts.len(),
            ids: missing_coverarts,
        };

        let missing_thumbnail =
            query_scalar!(r#"SELECT mbid FROM artists WHERE thumbnail_url IS NULL"#)
                .fetch_all(pool)
                .await?;

        let artist_without_thumbnail = MissingDataStats {
            count: missing_thumbnail.len(),
            ids: missing_thumbnail,
        };

        let total_track_count = query_scalar!(r#"SELECT count(*) FROM tracks"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        Ok(Stats {
            total_raw_scrobble_count,
            total_scrobble_count,
            total_track_count,
            total_releases_count,
            total_artists_count,
            unparsable_scrobbles,
            release_without_coverart,
            artist_without_thumbnail,
        })
    }
}
