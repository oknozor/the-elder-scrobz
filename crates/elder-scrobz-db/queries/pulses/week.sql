SELECT DATE_TRUNC('week', listened_at) AS period,
       COUNT(*)                        AS listens
FROM scrobbles
         JOIN scrobbles_raw ON scrobbles.source_id = scrobbles_raw.id
WHERE listened_at >= (NOW() - INTERVAL '12 weeks')
GROUP BY DATE_TRUNC('week', listened_at)
ORDER BY period;