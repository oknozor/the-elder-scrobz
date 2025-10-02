SELECT
    artist.mbid AS id,
    artist.name,
    artist.description,
    artist.thumbnail_url,
    artist.subsonic_id,
    MAX(raw.listened_at) AS last_listened_at,
    COUNT(DISTINCT raw.id) AS listens,
    COUNT(*) OVER () AS total
FROM scrobbles
    INNER JOIN scrobbles_raw AS raw ON scrobbles.source_id = raw.id
    INNER JOIN tracks AS track ON scrobbles.track_id = track.mbid
    INNER JOIN artists AS artist ON track.artist_mbid = artist.mbid
    INNER JOIN users AS u ON scrobbles.user_id = u.username
WHERE u.username = $1
GROUP BY
    artist.mbid,
    artist.name,
    artist.description,
    artist.thumbnail_url,
    artist.subsonic_id
ORDER BY listens DESC
LIMIT $2 OFFSET $3;
