SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       CASE
           WHEN last_7_days.artist_listened > 0 THEN
           (count(distinct artists.mbid) - last_7_days.artist_listened) * 100.0 /
           last_7_days.artist_listened
           ELSE null::float8
           END                      as artist_listened_percentage_increase,
       CASE
           WHEN last_7_days.track_listened > 0 THEN
           (count(distinct t.mbid) - last_7_days.track_listened) * 100.0 /
           last_7_days.track_listened
           ELSE null::float8
           END                      as track_listened_percentage_increase,
       CASE
           WHEN last_7_days.time_listened > 0 THEN
           (sum(t.length) - last_7_days.time_listened) * 100.0 /
           last_7_days.time_listened
           ELSE null::float8
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
                    WHERE s.created_at::date >= CURRENT_DATE - interval '7 days') as last_7_days ON true
WHERE s.created_at::date >= CURRENT_DATE - interval '7 days'
GROUP BY last_7_days.artist_listened, last_7_days.track_listened, last_7_days.time_listened;
