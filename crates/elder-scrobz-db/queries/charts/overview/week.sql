SELECT
    CASE
        WHEN last_7_days.artist_listened > 0
            THEN
                (count(DISTINCT artists.mbid) - last_7_days.artist_listened)
                * 100.0
                / last_7_days.artist_listened
        ELSE NULL::float8
    END AS artist_listened_percentage_increase,
    CASE
        WHEN last_7_days.track_listened > 0
            THEN
                (count(DISTINCT t.mbid) - last_7_days.track_listened) * 100.0
                / last_7_days.track_listened
        ELSE NULL::float8
    END AS track_listened_percentage_increase,
    CASE
        WHEN last_7_days.time_listened > 0
            THEN
                (sum(t.length) - last_7_days.time_listened) * 100.0
                / last_7_days.time_listened
        ELSE NULL::float8
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
        WHERE
            r.listened_at::date >= current_date - interval '14 days'
            AND r.listened_at::date < current_date - interval '7 days'
    ) AS last_7_days ON TRUE
WHERE r.listened_at::date >= current_date - interval '7 days'
GROUP BY
    last_7_days.artist_listened, last_7_days.track_listened, last_7_days.time_listened;
