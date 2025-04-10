SELECT extract(year from sr.listened_at)::text AS period,
       COUNT(sr.listened_at) AS listens
FROM scrobbles_raw sr
GROUP BY extract(year from sr.listened_at)::text
