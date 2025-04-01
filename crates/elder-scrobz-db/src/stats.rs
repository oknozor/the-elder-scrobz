use serde::Serialize;
use sqlx::{PgPool, query_scalar};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Stats {
    raw_scrobble_count: i64,
    scrobble_count: i64,
    unparsable_scrobble_count: i64,
    release_count: i64,
    track_count: i64,
    missing_coverarts_count: i64,
}

impl Stats {
    pub async fn get(pool: &PgPool) -> Result<Stats, sqlx::Error> {
        let raw_scrobble_count = query_scalar!(r#"SELECT count(*) FROM scrobbles_raw"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        let scrobble_count = query_scalar!(r#"SELECT count(*) FROM scrobbles"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        let release_count = query_scalar!(r#"SELECT count(*) FROM releases"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        let missing_coverarts_count =
            query_scalar!(r#"SELECT count(*) FROM releases WHERE cover_art_url IS NULL"#)
                .fetch_one(pool)
                .await?
                .unwrap();

        let track_count = query_scalar!(r#"SELECT count(*) FROM tracks"#)
            .fetch_one(pool)
            .await?
            .unwrap();

        Ok(Stats {
            raw_scrobble_count,
            scrobble_count,
            unparsable_scrobble_count: raw_scrobble_count - scrobble_count,
            release_count,
            track_count,
            missing_coverarts_count,
        })
    }
}
