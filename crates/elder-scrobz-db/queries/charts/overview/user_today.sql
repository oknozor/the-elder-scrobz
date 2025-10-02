SELECT
    CASE
        WHEN yesterday.artist_listened = 0 THEN null::float8
        ELSE
            (count(DISTINCT artists.mbid) - yesterday.artist_listened) * 100.0
            / yesterday.artist_listened
    END AS artist_listened_percentage_increase,
    CASE
        WHEN yesterday.track_listened = 0 THEN null::float8
        ELSE
            (count(DISTINCT t.mbid) - yesterday.track_listened) * 100.0
            / yesterday.track_listened
    END AS track_listened_percentage_increase,
    CASE
        WHEN yesterday.time_listened = 0 THEN null::float8
        ELSE
            (sum(t.length) - yesterday.time_listened) * 100.0
            / yesterday.time_listened
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
        WHERE
            s.user_id = $1
            AND s.created_at::date = current_date - 1
    ) AS yesterday ON true
WHERE s.created_at::date = current_date::date
GROUP BY
    yesterday.artist_listened, yesterday.track_listened, yesterday.time_listened;
