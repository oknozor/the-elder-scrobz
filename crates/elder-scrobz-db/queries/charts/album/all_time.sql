
SELECT release.mbid           AS id,
       release.name           AS name,
       release.subsonic_id    AS subsonic_id,
       release.cover_art_url  AS thumbnail_url,
       release.year           AS year,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens,
       COUNT(*) OVER()        AS total
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
         GROUP BY release.mbid, release.name, release.subsonic_id, release.cover_art_url, release.year
ORDER BY listens DESC
LIMIT $1 OFFSET $2;
