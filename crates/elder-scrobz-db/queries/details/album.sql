SELECT
    release.mbid AS id,
    release.subsonic_id,
    release.name,
    release.cover_art_url AS thumbnail_url,
    release.description,
    release.year,
    artist.name AS artist_name,
    artist.mbid AS artist_id,
    MAX(raw.listened_at) AS last_listened_at,
    COUNT(DISTINCT raw.id) AS listens,
    COALESCE(
        (
            SELECT
                JSON_AGG(
                    JSON_BUILD_OBJECT(
                        'mbid', tr.mbid,
                        'subsonic_id', tr.subsonic_id,
                        'name', tr.name,
                        'artist_name', tr.artist_display_name,
                        'number', tr.number,
                        'length', tr.length,
                        'playcount',
                        COALESCE(
                            (
                                SELECT
                                    JSON_AGG(
                                        JSON_BUILD_OBJECT(
                                            'username', pc.user_id,
                                            'count', pc.playcount
                                        )
                                    )
                                FROM (
                                    SELECT
                                        s.user_id,
                                        COUNT(*)::bigint AS playcount
                                    FROM scrobbles AS s
                                    WHERE s.track_id = tr.mbid
                                    GROUP BY s.user_id
                                    ORDER BY COUNT(*) DESC
                                    LIMIT 10
                                ) AS pc
                            ),
                            '[]'::json
                        )
                    )
                    ORDER BY tr.number
                )
            FROM tracks AS tr
            WHERE tr.release_mbid = release.mbid
        ),
        '[]'::json
    ) AS "tracks: Json<Vec<AlbumTrackWithPlayCount>>"
FROM scrobbles
    INNER JOIN scrobbles_raw AS raw ON scrobbles.source_id = raw.id
    INNER JOIN tracks AS track ON scrobbles.track_id = track.mbid
    INNER JOIN releases AS release ON track.release_mbid = release.mbid
    INNER JOIN artists AS artist ON release.artist_mbid = artist.mbid
WHERE release.mbid = $1
GROUP BY
    release.mbid, release.subsonic_id, release.name, release.cover_art_url,
    release.description, release.year, artist.mbid, artist.name
