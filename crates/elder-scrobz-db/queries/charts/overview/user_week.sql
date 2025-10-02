SELECT
    CASE
        WHEN last_7_days.artist_listened > 0
            THEN
                (count(DISTINCT artists.mbid) - last_7_days.artist_listened)
                * 100.0
                / last_7_days.artist_listened
        ELSE null::float8
    END AS artist_listened_percentage_increase,
    CASE
        WHEN last_7_days.track_listened > 0
            THEN
                (count(DISTINCT t.mbid) - last_7_days.track_listened) * 100.0
                / last_7_days.track_listened
        ELSE null::float8
    END AS track_listened_percentage_increase,
    CASE
        WHEN last_7_days.time_listened > 0
            THEN
                (sum(t.length) - last_7_days.time_listened) * 100.0
                / last_7_days.time_listened
        ELSE null::float8
    END AS time_listened_percentage_increase,
    count(DISTINCT artists.mbid) AS artist_listened,
    count(DISTINCT t.mbid) AS track_listened,
    sum(t.length) AS time_listened
FROM artists
    INNER JOIN public.tracks AS t ON artists.mbid = t.artist_mbid
    INNER JOIN public.scrobbles AS s ON t.mbid = s.track_id AND s.user_id = $1
    LEFT JOIN (SELECT
        count(DISTINCT artists.mbid) AS artist_listened,
        count(DISTINCT t.mbid) AS track_listened,
        sum(t.length) AS time_listened
    FROM artists
        INNER JOIN public.tracks AS t ON artists.mbid = t.artist_mbid
        INNER JOIN public.scrobbles AS s ON t.mbid = s.track_id
    WHERE s.created_at::date >= current_date - interval '7 days') AS last_7_days
        ON true
WHERE s.created_at::date >= current_date - interval '7 days'
GROUP BY
    last_7_days.artist_listened, last_7_days.track_listened, last_7_days.time_listened;
