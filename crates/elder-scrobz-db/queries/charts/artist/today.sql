SELECT artist.mbid            AS id,
       artist.name            AS name,
       artist.description     AS description,
       artist.thumbnail_url   AS thumbnail_url,
       artist.subsonic_id     AS subsonic_id,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens,
       COUNT(*) OVER ()       AS total
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN artists artist ON track.artist_mbid = artist.mbid
WHERE DATE(listened_at) = CURRENT_DATE
GROUP BY artist.mbid,
         artist.name,
         artist.description,
         artist.thumbnail_url,
         artist.subsonic_id
ORDER BY listens DESC
LIMIT $1 OFFSET $2;
