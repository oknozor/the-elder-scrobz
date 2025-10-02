SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       CASE
           WHEN last_30_days.artist_listened = 0 THEN NULL::float8
           ELSE (count(distinct artists.mbid) - last_30_days.artist_listened) * 100.0 /
                last_30_days.artist_listened
           END as artist_listened_percentage_increase,
       CASE
           WHEN last_30_days.track_listened = 0 THEN NULL::float8
           ELSE (count(distinct t.mbid) - last_30_days.track_listened) * 100.0 /
                last_30_days.track_listened
           END as track_listened_percentage_increase,
       CASE
           WHEN last_30_days.time_listened = 0 THEN NULL::float8
           ELSE (sum(t.length) - last_30_days.time_listened) * 100.0 /
                last_30_days.time_listened
           END as time_listened_percentage_increase
FROM artists
JOIN public.tracks t ON artists.mbid = t.artist_mbid
JOIN public.scrobbles s ON t.mbid = s.track_id
LEFT JOIN (
    SELECT count(distinct artists.mbid) as artist_listened,
           count(distinct t.mbid)       as track_listened,
           sum(t.length)                as time_listened
    FROM artists
    JOIN public.tracks t ON artists.mbid = t.artist_mbid
    JOIN public.scrobbles s ON t.mbid = s.track_id
    WHERE s.created_at::date >= CURRENT_DATE - interval '60 days'
      AND s.created_at::date <  CURRENT_DATE - interval '30 days'
) as last_30_days ON true
WHERE s.created_at::date >= CURRENT_DATE - interval '30 days'
GROUP BY last_30_days.artist_listened, last_30_days.track_listened, last_30_days.time_listened;
