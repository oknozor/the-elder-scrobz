SELECT
    NULL::double precision AS artist_listened_percentage_increase,
    NULL::double precision AS track_listened_percentage_increase,
    NULL::double precision AS time_listened_percentage_increase,
    count(DISTINCT artists.mbid) AS artist_listened,
    count(DISTINCT t.mbid) AS track_listened,
    sum(t.length) AS time_listened
FROM artists
    INNER JOIN public.tracks AS t ON artists.mbid = t.artist_mbid
    INNER JOIN public.scrobbles AS s ON t.mbid = s.track_id
WHERE s.user_id = $1
