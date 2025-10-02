SELECT count(distinct artists.mbid) as artist_listened,
       count(distinct t.mbid)       as track_listened,
       sum(t.length)                as time_listened,
       NULL::double precision as artist_listened_percentage_increase,
       NULL::double precision as track_listened_percentage_increase,
       NULL::double precision as time_listened_percentage_increase
FROM artists
         JOIN public.tracks t on artists.mbid = t.artist_mbid
         JOIN public.scrobbles s on t.mbid = s.track_id
         WHERE s.user_id = $1
