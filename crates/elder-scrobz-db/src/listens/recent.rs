use crate::PgPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, ToSchema)]
pub struct RecentListen {
    #[serde(rename = "id")]
    pub track_id: Option<String>,
    #[serde(rename = "user")]
    pub username: Option<String>,
    #[serde(rename = "imageUrl")]
    pub cover_art_url: Option<String>,
    #[serde(rename = "playedAt")]
    pub listened_at: Option<DateTime<Utc>>,
    #[serde(rename = "artist")]
    pub artist_name: Option<String>,
    #[serde(rename = "title")]
    pub track_name: Option<String>,
    #[serde(rename = "duration")]
    pub duration: Option<i32>,
}

pub async fn get_recent_listens(
    page: i64,
    page_size: i64,
    pool: &PgPool,
) -> Result<Vec<RecentListen>, Error> {
    let offset = (page - 1) * page_size;

    sqlx::query_as!(
        RecentListen,
        r#"
        SELECT 
            t.mbid as track_id,
            u.username as username,
            r.cover_art_url as cover_art_url,
            sr.listened_at as listened_at,
            COALESCE(t.artist_display_name, a.name) as artist_name,
            t.name as track_name,
            t.length as duration
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
        page_size,
        offset
    )
    .fetch_all(pool)
    .await
}
