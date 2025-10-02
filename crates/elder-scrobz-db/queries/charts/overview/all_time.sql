SELECT
    NULL::double precision AS artist_listened_percentage_increase,
    NULL::double precision AS track_listened_percentage_increase,
    NULL::double precision AS time_listened_percentage_increase,
    count(DISTINCT artists.mbid) AS artist_listened,
    count(DISTINCT t.mbid) AS track_listened,
    sum(t.length) AS time_listened
FROM scrobbles AS s
    INNER JOIN tracks AS t ON s.track_id = t.mbid
    INNER JOIN artists ON t.artist_mbid = artists.mbid
