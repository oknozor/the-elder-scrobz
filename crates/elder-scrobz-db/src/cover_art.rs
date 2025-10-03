use sqlx::PgPool;

pub async fn get(release_mbid: &str, db: &PgPool) -> Result<Option<String>, sqlx::Error> {
    let query = "SELECT cover_art_url FROM releases WHERE mbid = $1";
    let result = sqlx::query_scalar(query)
        .bind(release_mbid)
        .fetch_optional(db)
        .await?;

    Ok(result)
}
