SELECT track_id              as id,
       track.name            as name,
       track.length          as length,
       release.mbid          as release_mbid,
       release.name          as release_name,
       release.cover_art_url as thumbnail_url,
       track.subsonic_id     as subsonic_id,
       release.subsonic_id   as release_subsonic_id,
       count(*)              as listens,
       COUNT(*) OVER()       as total
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
         JOIN users u on scrobbles.user_id = u.username
WHERE DATE_TRUNC('month', listened_at) = DATE_TRUNC('month', NOW())
  AND u.username = $1

GROUP BY track_id, track.name, track.length, release.mbid, release.name, release.cover_art_url, track.subsonic_id, release.subsonic_id
ORDER BY listens DESC
LIMIT $2 OFFSET $3;
