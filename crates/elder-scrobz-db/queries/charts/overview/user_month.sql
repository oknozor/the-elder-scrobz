SELECT
    CASE
        WHEN last_30_days.artist_listened = 0 THEN NULL::float8
        ELSE
            (count(DISTINCT artists.mbid) - last_30_days.artist_listened)
            * 100.0
            / last_30_days.artist_listened
    END AS artist_listened_percentage_increase,
    CASE
        WHEN last_30_days.track_listened = 0 THEN NULL::float8
        ELSE
            (count(DISTINCT t.mbid) - last_30_days.track_listened) * 100.0
            / last_30_days.track_listened
    END AS track_listened_percentage_increase,
    CASE
        WHEN last_30_days.time_listened = 0 THEN NULL::float8
        ELSE
            (sum(t.length) - last_30_days.time_listened) * 100.0
            / last_30_days.time_listened
    END AS time_listened_percentage_increase,
    count(DISTINCT artists.mbid) AS artist_listened,
    count(DISTINCT t.mbid) AS track_listened,
    sum(t.length) AS time_listened
FROM artists
    INNER JOIN public.tracks AS t ON artists.mbid = t.artist_mbid
    INNER JOIN public.scrobbles AS s ON t.mbid = s.track_id AND s.user_id = $1
    LEFT JOIN (
        SELECT
            count(DISTINCT artists.mbid) AS artist_listened,
            count(DISTINCT t.mbid) AS track_listened,
            sum(t.length) AS time_listened
        FROM artists
            INNER JOIN public.tracks AS t ON artists.mbid = t.artist_mbid
            INNER JOIN public.scrobbles AS s ON t.mbid = s.track_id
        WHERE s.created_at::date >= current_date - interval '30 days'
    ) AS last_30_days
        ON TRUE
WHERE s.created_at::date >= current_date - interval '30 days'
GROUP BY
    last_30_days.artist_listened, last_30_days.track_listened, last_30_days.time_listened;
