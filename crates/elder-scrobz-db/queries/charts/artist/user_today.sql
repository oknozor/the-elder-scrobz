SELECT artist.mbid            AS id,
       artist.name            AS name,
       artist.description     AS description,
       artist.thumbnail_url   AS thumbnail_url,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens,
       COUNT(*) OVER ()       AS total
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN artists artist ON track.artist_mbid = artist.mbid
         JOIN users u on scrobbles.user_id = u.username
WHERE DATE(listened_at) = CURRENT_DATE
  AND u.username = $1
GROUP BY artist.mbid, artist.name
ORDER BY listens DESC
LIMIT $2 OFFSET $3;
