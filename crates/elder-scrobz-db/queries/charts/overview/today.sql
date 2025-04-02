SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       CASE
           WHEN yesterday.artist_listened = 0 THEN null::float8
           ELSE (count(distinct artists.mbid) - yesterday.artist_listened) * 100.0 /
                yesterday.artist_listened
           END                      as artist_listened_percentage_increase,
       CASE
           WHEN yesterday.track_listened = 0 THEN null::float8
           ELSE (count(distinct t.mbid) - yesterday.track_listened) * 100.0 /
                yesterday.track_listened
           END                      as track_listened_percentage_increase,
       CASE
           WHEN yesterday.time_listened = 0 THEN null::float8
           ELSE (sum(t.length) - yesterday.time_listened) * 100.0 /
                yesterday.time_listened
           END                      as time_listened_percentage_increase
FROM artists
         JOIN public.tracks t on artists.mbid = t.artist_mbid
         JOIN public.scrobbles s on t.mbid = s.track_id
         LEFT JOIN (SELECT count(distinct artists.mbid) as artist_listened,
                           count(distinct t.mbid)       as track_listened,
                           sum(t.length)                as time_listened
                    FROM artists
                             JOIN public.tracks t on artists.mbid = t.artist_mbid
                             JOIN public.scrobbles s on t.mbid = s.track_id
                    WHERE s.created_at::date = CURRENT_DATE - 1) as yesterday ON true
WHERE s.created_at::date = CURRENT_DATE::date
GROUP BY yesterday.artist_listened, yesterday.track_listened, yesterday.time_listened;
