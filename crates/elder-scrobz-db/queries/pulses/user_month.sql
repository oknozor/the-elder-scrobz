WITH all_days AS (SELECT generate_series(
                                 CURRENT_DATE - interval '29 days',
                                 CURRENT_DATE,
                                 interval '1 day'
                         )::date AS day)
SELECT extract(day from all_days.day)::text || '/' || LPAD(extract(month from all_days.day)::text, 2, '0') AS period,
       COALESCE(COUNT(scrobbles_raw.listened_at), 0) AS listens
FROM all_days
LEFT JOIN scrobbles_raw
  ON DATE_TRUNC('day', scrobbles_raw.listened_at) = all_days.day
 AND scrobbles_raw.user_id = $1
WHERE scrobbles_raw.listened_at >= CURRENT_DATE - interval '29 days'
   OR scrobbles_raw.listened_at IS NULL
GROUP BY all_days.day
ORDER BY all_days.day;
