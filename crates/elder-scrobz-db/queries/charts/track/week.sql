SELECT
    track_id as track_id,
    track.name as track_name,
    track.length as track_length,
    release.mbid as release_mbid,
    release.name as release_name,
    release.cover_art_url as cover_art_url,
    count(*) as listens,
    COUNT(*) OVER () as total
FROM
    scrobbles
    JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
    JOIN tracks track ON track.mbid = scrobbles.track_id
    JOIN releases release ON track.release_mbid = release.mbid
WHERE
    DATE_TRUNC ('week', listened_at) = DATE_TRUNC ('week', NOW ())
GROUP BY
    track_id,
    track.name,
    track.length,
    release.mbid,
    release.mbid,
    release.name,
    release.cover_art_url
ORDER BY
    listens DESC
LIMIT $1 OFFSET $2;

