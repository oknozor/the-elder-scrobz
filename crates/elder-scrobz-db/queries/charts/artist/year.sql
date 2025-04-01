SELECT artist.mbid            AS artist_id,
       artist.name            AS artist_name,
       artist.description     AS description,
       artist.thumbnail_url   AS thumbnail_url,
       MAX(raw.listened_at)   AS last_listened_at,
       COUNT(DISTINCT raw.id) AS listens,
       COUNT(*) OVER ()       AS total
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN artists artist ON track.artist_mbid = artist.mbid
WHERE EXTRACT(
              YEAR
              FROM
              listened_at
      ) = EXTRACT(
              YEAR
              FROM
              NOW()
          )
GROUP BY artist.mbid,
         artist.name
ORDER BY listens DESC
LIMIT 10;
