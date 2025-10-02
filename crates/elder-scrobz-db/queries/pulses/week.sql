WITH weekdays AS (SELECT CASE
                            WHEN d = current_date THEN TO_CHAR(d, 'Day') || ' (Today)'
                            ELSE TO_CHAR(d, 'Day')
                         END AS period,
                         EXTRACT(ISODOW FROM d) AS isodow, d AS date_val
                  FROM generate_series(
                               current_date - interval '6 days',
                               current_date,
                               interval '1 day'
                       ) d)
SELECT w.period,
       COALESCE(COUNT(scrobbles_raw.id), 0) AS listens
FROM weekdays w
         LEFT JOIN scrobbles_raw
                   ON w.isodow = EXTRACT(ISODOW FROM scrobbles_raw.listened_at)
                       AND scrobbles_raw.listened_at >= current_date - interval '6 days'
GROUP BY w.period, w.isodow, w.date_val
ORDER BY w.date_val;
