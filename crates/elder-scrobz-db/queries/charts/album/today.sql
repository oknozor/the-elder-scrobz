SELECT release.mbid           AS id,
       release.name           AS name,
       release.cover_art_url  AS thumbnail_url,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens,
       count(*) OVER()        as total
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
WHERE DATE(listened_at) = CURRENT_DATE
GROUP BY release.mbid, release.name, release.cover_art_url
ORDER BY listens DESC
LIMIT $1 OFFSET $2;
