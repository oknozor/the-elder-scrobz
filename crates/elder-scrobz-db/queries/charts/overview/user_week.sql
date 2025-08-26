
SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       CASE
           WHEN last_week.artist_listened > 0 THEN
               (count(distinct artists.mbid) - last_week.artist_listened) * 100.0 /
               last_week.artist_listened
           ELSE null::float8
           END                      as artist_listened_percentage_increase,
       CASE
           WHEN last_week.track_listened > 0 THEN
               (count(distinct t.mbid) - last_week.track_listened) * 100.0 /
               last_week.track_listened
           ELSE null::float8
           END                      as track_listened_percentage_increase,
       CASE
           WHEN last_week.time_listened > 0 THEN
               (sum(t.length) - last_week.time_listened) * 100.0 /
               last_week.time_listened
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
                    WHERE s.user_id = $1
                      AND s.created_at::date >= date_trunc('week', CURRENT_DATE - interval '1 week')
                      AND s.created_at::date < date_trunc('week', CURRENT_DATE)) as last_week ON true
WHERE s.created_at::date >= date_trunc('week', CURRENT_DATE)
GROUP BY last_week.artist_listened, last_week.track_listened, last_week.time_listened;
