SELECT
    CASE
        WHEN last_12_month.artist_listened = 0 THEN NULL::float8
        ELSE
            (count(DISTINCT artists.mbid) - last_12_month.artist_listened)
            * 100.0
            / last_12_month.artist_listened
    END AS artist_listened_percentage_increase,
    CASE
        WHEN last_12_month.track_listened = 0 THEN NULL::float8
        ELSE
            (count(DISTINCT t.mbid) - last_12_month.track_listened) * 100.0
            / last_12_month.track_listened
    END AS track_listened_percentage_increase,
    CASE
        WHEN last_12_month.time_listened = 0 THEN NULL::float8
        ELSE
            (sum(t.length) - last_12_month.time_listened) * 100.0
            / last_12_month.time_listened
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
        WHERE s.created_at::date >= current_date - interval '12 months'
    ) AS last_12_month
        ON TRUE
WHERE s.created_at::date >= current_date - interval '12 months'
GROUP BY
    last_12_month.artist_listened, last_12_month.track_listened, last_12_month.time_listened;
