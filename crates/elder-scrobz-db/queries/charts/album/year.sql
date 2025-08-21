SELECT release.mbid           AS id,
       release.name           AS name,
       release.cover_art_url  AS cover_art_url,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens,
       COUNT(*) OVER()        AS total
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
WHERE EXTRACT(YEAR FROM listened_at) = EXTRACT(YEAR FROM NOW())
GROUP BY release.mbid, release.name, release.cover_art_url
ORDER BY listens DESC
LIMIT $1 OFFSET $2;
