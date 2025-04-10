WITH weekdays AS (SELECT TO_CHAR(d, 'Day') AS period, EXTRACT(ISODOW FROM d) AS isodow
                  FROM generate_series(
                               date_trunc('week', current_date),
                               date_trunc('week', current_date) + interval '6 days',
                               interval '1 day'
                       ) d)
SELECT w.period,
       COALESCE(COUNT(scrobbles_raw.id), 0) AS listens
FROM weekdays w
         LEFT JOIN scrobbles_raw
                   ON w.isodow = EXTRACT(ISODOW FROM scrobbles_raw.listened_at)
                       AND scrobbles_raw.listened_at >= date_trunc('week', current_date)
WHERE scrobbles_raw.user_id = $1
GROUP BY w.period, w.isodow
ORDER BY w.isodow;