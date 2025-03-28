SELECT track_id              as track_id,
       track.name            as track_name,
       track.length          as track_length,
       release.name          as release_name,
       release.cover_art_url as cover_art_url,
       count(*)              as listens
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
         JOIN users u on scrobbles.user_id = u.username
WHERE DATE_TRUNC('month', listened_at) = DATE_TRUNC('month', NOW())
  AND u.username = $1

GROUP BY track_id, track.name, track.length, release.name, release.cover_art_url
ORDER BY listens DESC
LIMIT 10