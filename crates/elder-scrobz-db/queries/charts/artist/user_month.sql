SELECT artist.mbid            AS artist_id,
       artist.name            AS artist_name,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN artists artist ON track.artist_mbid = artist.mbid
         JOIN users u on scrobbles.user_id = u.username
WHERE DATE_TRUNC('month', raw.listened_at) = DATE_TRUNC('month', NOW())
  AND u.username = $1
GROUP BY artist.mbid, artist.name
ORDER BY listens DESC
LIMIT 10;