SELECT DATE_TRUNC('month', listened_at) AS period,
       COUNT(*)                         AS listens
FROM scrobbles
         JOIN scrobbles_raw on scrobbles.source_id = scrobbles_raw.id
WHERE listened_at >= (NOW() - INTERVAL '12 months')
    AND scrobbles.user_id = $1
GROUP BY DATE_TRUNC('month', listened_at)
ORDER BY period;