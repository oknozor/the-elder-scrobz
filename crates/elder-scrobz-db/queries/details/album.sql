SELECT
  release.mbid            AS id,
  release.subsonic_id     AS subsonic_id,
  release.name            AS name,
  release.cover_art_url   AS thumbnail_url,
  release.description     AS description,
  release.year            AS year,
  artist.name             AS artist_name,
  artist.mbid             AS artist_id,
  MAX(raw.listened_at)    AS last_listened_at,
  COUNT(DISTINCT raw.id)  AS listens,
  COALESCE(
    (
      SELECT json_agg(
        json_build_object(
          'mbid', tr.mbid,
          'subsonic_id', tr.subsonic_id,
          'name', tr.name,
          'number', tr.number,
          'length', tr.length,
          'playcount',
            COALESCE(
              (
                SELECT json_agg(
                  json_build_object(
                    'username', pc.user_id,
                    'count', pc.playcount
                  )
                )
                FROM (
                  SELECT s.user_id, COUNT(*)::bigint AS playcount
                  FROM scrobbles s
                  WHERE s.track_id = tr.mbid
                  GROUP BY s.user_id
                  ORDER BY COUNT(*) DESC
                  LIMIT 10
                ) pc
              ),
              '[]'::json
            )
        )
        ORDER BY tr.number
      )
      FROM tracks tr
      WHERE tr.release_mbid = release.mbid
    ),
    '[]'::json
  ) AS "tracks: Json<Vec<AlbumTrackWithPlayCount>>"
FROM scrobbles
JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
JOIN tracks track ON track.mbid = scrobbles.track_id
JOIN releases release ON track.release_mbid = release.mbid
JOIN artists artist ON artist.mbid = release.artist_mbid
WHERE release.mbid = $1
GROUP BY release.mbid, release.subsonic_id, release.name, release.cover_art_url,
         release.description, release.year, artist.mbid, artist.name
