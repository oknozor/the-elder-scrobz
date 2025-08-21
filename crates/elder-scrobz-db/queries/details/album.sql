SELECT artist.mbid            as id,
       artist.name            as name,
       release.mbid           AS release_id,
       release.name           as release_name,
       release.cover_art_url  as thumbnail_url,
       release.description    as description,
       max(raw.listened_at)   as last_listened_at,
       count(distinct raw.id) as listens
FROM scrobbles
         JOIN scrobbles_raw raw ON scrobbles.source_id = raw.id
         JOIN tracks track ON track.mbid = scrobbles.track_id
         JOIN releases release ON track.release_mbid = release.mbid
         JOIN artists artist ON artist.mbid = release.artist_mbid
WHERE release.mbid = $1
GROUP BY release.mbid, release.name, release.cover_art_url, artist.mbid, artist.name
