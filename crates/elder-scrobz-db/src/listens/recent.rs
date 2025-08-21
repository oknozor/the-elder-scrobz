use crate::PgPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, ToSchema)]
pub struct RecentListen {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "user")]
    pub username: Option<String>,
    pub thumbnail_url: Option<String>,
    pub listened_at: Option<DateTime<Utc>>,
    pub artist_id: Option<String>,
    pub artist_name: Option<String>,
    pub duration: Option<i32>,
    #[serde(skip)]
    pub total: Option<i64>,
}

pub async fn get_recent_listens(
    limit: i64,
    offset: i64,
    pool: &PgPool,
) -> Result<(i64, Vec<RecentListen>), Error> {
    let listens = sqlx::query_as!(
        RecentListen,
        r#"
        SELECT
            t.mbid as id,
            u.username as username,
            r.cover_art_url as thumbnail_url,
            sr.listened_at as listened_at,
            a.mbid as artist_id,
            COALESCE(t.artist_display_name, a.name) as artist_name,
            t.name as name,
            t.length as duration,
            count(*) over () as total
        FROM
            scrobbles s
            JOIN scrobbles_raw sr ON s.source_id = sr.id
            JOIN users u ON s.user_id = u.username
            JOIN tracks t ON s.track_id = t.mbid
            JOIN releases r ON t.release_mbid = r.mbid
            JOIN artists a ON t.artist_mbid = a.mbid
        ORDER BY
            sr.listened_at DESC
        LIMIT $1
        OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    let total = listens.first().and_then(|r| r.total).unwrap_or_default();
    Ok((total, listens))
}
