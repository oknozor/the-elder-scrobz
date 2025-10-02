SELECT
    release.mbid AS id,
    release.name,
    release.subsonic_id,
    release.cover_art_url AS thumbnail_url,
    release.year,
    MAX(raw.listened_at) AS last_listened_at,
    COUNT(DISTINCT raw.id) AS listens,
    COUNT(*) OVER () AS total
FROM scrobbles
    INNER JOIN scrobbles_raw AS raw ON scrobbles.source_id = raw.id
    INNER JOIN tracks AS track ON scrobbles.track_id = track.mbid
    INNER JOIN releases AS release ON track.release_mbid = release.mbid
    INNER JOIN users AS u ON scrobbles.user_id = u.username
WHERE
    EXTRACT(YEAR FROM listened_at) = EXTRACT(YEAR FROM NOW())
    AND u.username = $1
GROUP BY
    release.mbid,
    release.name,
    release.subsonic_id,
    release.cover_art_url,
    release.year
ORDER BY listens DESC
LIMIT $2 OFFSET $3;
