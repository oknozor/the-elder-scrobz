
SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       CASE
           WHEN last_year.artist_listened = 0 THEN NULL::float8
           ELSE (count(distinct artists.mbid) - last_year.artist_listened) * 100.0 /
                last_year.artist_listened
           END                      as artist_listened_percentage_increase,
       CASE
           WHEN last_year.track_listened = 0 THEN NULL::float8
           ELSE (count(distinct t.mbid) - last_year.track_listened) * 100.0 /
                last_year.track_listened
           END                      as track_listened_percentage_increase,
       CASE
           WHEN last_year.time_listened = 0 THEN NULL::float8
           ELSE (sum(t.length) - last_year.time_listened) * 100.0 /
                last_year.time_listened
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
                    WHERE s.user_id = $1
                      AND s.created_at::date >= date_trunc('year', CURRENT_DATE - interval '1 year')
                      AND s.created_at::date < date_trunc('year', CURRENT_DATE)) as last_year ON true
WHERE s.created_at::date >= date_trunc('year', CURRENT_DATE)
GROUP BY last_year.artist_listened, last_year.track_listened, last_year.time_listened;
