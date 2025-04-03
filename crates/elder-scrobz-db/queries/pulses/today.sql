WITH time_series AS (SELECT generate_series(
                                    DATE_TRUNC('day', NOW()),
                                    DATE_TRUNC('day', NOW()) + INTERVAL '23 hour',
                                    '1 hour'
                            ) AS period)
SELECT ts.period::text                          AS period,
       COALESCE(COUNT(sr.listened_at), 0) AS listens
FROM time_series ts
         LEFT JOIN scrobbles_raw sr
                   ON DATE_TRUNC('hour', sr.listened_at) = ts.period
WHERE sr.listened_at >= DATE_TRUNC('day', NOW())
   OR sr.listened_at IS NULL
GROUP BY ts.period
ORDER BY ts.period;