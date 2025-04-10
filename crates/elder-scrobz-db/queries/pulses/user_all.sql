SELECT extract(year from sr.listened_at)::text AS period,
       COUNT(sr.listened_at) AS listens
FROM scrobbles_raw sr
WHERE sr.user_id = $1
GROUP BY extract(year from sr.listened_at)::text
