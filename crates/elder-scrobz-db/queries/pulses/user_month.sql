WITH all_days AS (SELECT generate_series(
                                 date_trunc('month', CURRENT_DATE),
                                 date_trunc('month', CURRENT_DATE) + interval '1 month' - interval '1 day',
                                 interval '1 day'
                         )::date AS day)
SELECT extract(day from all_days.day)::text                            AS period,
       COALESCE(COUNT(scrobbles_raw.listened_at), 0) AS listens
FROM all_days
         LEFT JOIN scrobbles_raw
                   ON DATE_TRUNC('day', scrobbles_raw.listened_at) = all_days.day
WHERE (scrobbles_raw.listened_at >= date_trunc('month', CURRENT_DATE)
   OR scrobbles_raw.listened_at IS NULL) and scrobbles_raw.user_id = $1
GROUP BY all_days.day
ORDER BY all_days.day;