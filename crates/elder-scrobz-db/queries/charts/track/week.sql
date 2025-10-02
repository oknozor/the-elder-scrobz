SELECT
    track_id AS id,
    track.name,
    track.length,
    release.mbid AS release_mbid,
    release.name AS release_name,
    release.cover_art_url AS thumbnail_url,
    track.subsonic_id,
    release.subsonic_id AS release_subsonic_id,
    count(*) AS listens,
    count(*) OVER () AS total
FROM
    scrobbles
    INNER JOIN scrobbles_raw AS raw ON scrobbles.source_id = raw.id
    INNER JOIN tracks AS track ON scrobbles.track_id = track.mbid
    INNER JOIN releases AS release ON track.release_mbid = release.mbid
WHERE
    date_trunc('week', listened_at) = date_trunc('week', now())
GROUP BY
    track_id,
    track.name,
    track.length,
    release.mbid,
    release.name,
    release.cover_art_url,
    track.subsonic_id,
    release.subsonic_id
ORDER BY
    listens DESC
LIMIT $1 OFFSET $2;
