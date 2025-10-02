SELECT
    extract(YEAR FROM sr.listened_at)::text AS period,
    count(sr.listened_at) AS listens
FROM scrobbles_raw AS sr
GROUP BY extract(YEAR FROM sr.listened_at)::text
