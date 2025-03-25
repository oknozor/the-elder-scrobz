SELECT release.mbid           AS release_id,
       release.name           AS release_name,
       release.cover_art_url  AS cover_art_url,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
WHERE EXTRACT(YEAR FROM listened_at) = EXTRACT(YEAR FROM NOW())
  AND scrobbles.user_id = $1
GROUP BY release.mbid, release.name, release.cover_art_url
ORDER BY listens DESC
LIMIT 10;