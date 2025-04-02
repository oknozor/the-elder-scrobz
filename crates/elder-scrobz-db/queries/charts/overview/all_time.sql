SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       0 as artist_listened_percentage_increase,
       0 as track_listened_percentage_increase,
       0 as time_listened_percentage_increase
FROM artists
         JOIN public.tracks t on artists.mbid = t.artist_mbid
         JOIN public.scrobbles s on t.mbid = s.track_id