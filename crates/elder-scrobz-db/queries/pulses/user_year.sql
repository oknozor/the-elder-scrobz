SELECT DATE_TRUNC('year', listened_at) AS period,
       COUNT(*)                        AS listens
FROM scrobbles
         JOIN scrobbles_raw ON scrobbles.source_id = scrobbles_raw.id
WHERE listened_at >= (NOW() - INTERVAL '12 years')
  AND scrobbles.user_id = $1
GROUP BY DATE_TRUNC('year', listened_at)
ORDER BY period;
