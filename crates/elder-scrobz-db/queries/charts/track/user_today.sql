SELECT track_id              as track_id,
       track.name            as track_name,
       track.length          as track_length,
       release.name          as release_name,
       release.cover_art_url as cover_art_url,
       raw.listened_at       as listened_at,
       count(*)              as listens
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
         JOIN users u on scrobbles.user_id = u.id
WHERE DATE(listened_at) = CURRENT_DATE
  AND u.username = $1
GROUP BY track_id, track.name, track.length, release.name, release.cover_art_url, raw.listened_at
ORDER BY listens DESC
LIMIT 10