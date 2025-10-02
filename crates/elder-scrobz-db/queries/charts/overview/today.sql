SELECT
    CASE
        WHEN yesterday.artist_listened = 0 THEN NULL::float8
        ELSE
            (count(DISTINCT artists.mbid) - yesterday.artist_listened) * 100.0
            / yesterday.artist_listened
    END AS artist_listened_percentage_increase,
    CASE
        WHEN yesterday.track_listened = 0 THEN NULL::float8
        ELSE
            (count(DISTINCT t.mbid) - yesterday.track_listened) * 100.0
            / yesterday.track_listened
    END AS track_listened_percentage_increase,
    CASE
        WHEN yesterday.time_listened = 0 THEN NULL::float8
        ELSE
            (sum(t.length) - yesterday.time_listened) * 100.0
            / yesterday.time_listened
    END AS time_listened_percentage_increase,
    count(DISTINCT artists.mbid) AS artist_listened,
    count(DISTINCT t.mbid) AS track_listened,
    sum(t.length) AS time_listened
FROM scrobbles AS s
    INNER JOIN scrobbles_raw AS r ON s.source_id = r.id
    INNER JOIN tracks AS t ON s.track_id = t.mbid
    INNER JOIN artists ON t.artist_mbid = artists.mbid
    LEFT JOIN (
        SELECT
            count(DISTINCT artists.mbid) AS artist_listened,
            count(DISTINCT t.mbid) AS track_listened,
            sum(t.length) AS time_listened
        FROM scrobbles AS s
            INNER JOIN scrobbles_raw AS r ON s.source_id = r.id
            INNER JOIN tracks AS t ON s.track_id = t.mbid
            INNER JOIN artists ON t.artist_mbid = artists.mbid
        WHERE r.listened_at::date = current_date - 1
    ) AS yesterday ON TRUE
WHERE r.listened_at::date = current_date
GROUP BY
    yesterday.artist_listened,
    yesterday.track_listened,
    yesterday.time_listened;
