SELECT DATE_TRUNC('day', listened_at)::text AS period,
       COUNT(*)                         AS listens
FROM scrobbles
         JOIN scrobbles_raw on scrobbles.source_id = scrobbles_raw.id
WHERE listened_at >= date_trunc('month', CURRENT_DATE)
    AND scrobbles.user_id = $1
GROUP BY DATE_TRUNC('day', listened_at)
ORDER BY period;
