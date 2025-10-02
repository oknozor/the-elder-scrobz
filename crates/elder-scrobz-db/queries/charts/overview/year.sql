SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       CASE
           WHEN last_12_month.artist_listened = 0 THEN NULL::float8
           ELSE (count(distinct artists.mbid) - last_12_month.artist_listened) * 100.0 /
           last_12_month.artist_listened
           END                      as artist_listened_percentage_increase,
       CASE
           WHEN last_12_month.track_listened = 0 THEN NULL::float8
           ELSE (count(distinct t.mbid) - last_12_month.track_listened) * 100.0 /
           last_12_month.track_listened
           END                      as track_listened_percentage_increase,
       CASE
           WHEN last_12_month.time_listened = 0 THEN NULL::float8
           ELSE (sum(t.length) - last_12_month.time_listened) * 100.0 /
           last_12_month.time_listened
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
                    WHERE s.created_at::date >= CURRENT_DATE - interval '12 months') as last_12_month ON true
WHERE s.created_at::date >= CURRENT_DATE - interval '12 months'
GROUP BY last_12_month.artist_listened, last_12_month.track_listened, last_12_month.time_listened;
