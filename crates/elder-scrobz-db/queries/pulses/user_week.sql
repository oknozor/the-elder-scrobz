WITH weekdays AS (
    SELECT
        CASE
            WHEN d = current_date THEN to_char(d, 'Day') || ' (Today)'
            ELSE to_char(d, 'Day')
        END AS period,
        d AS date_val,
        extract(ISODOW FROM d) AS isodow
    FROM
        generate_series(
            current_date - interval '6 days',
            current_date,
            interval '1 day'
        ) AS d
)

SELECT
    w.period,
    coalesce(count(scrobbles_raw.id), 0) AS listens
FROM weekdays AS w
    LEFT JOIN scrobbles_raw
        ON
            w.isodow = extract(ISODOW FROM scrobbles_raw.listened_at)
            AND scrobbles_raw.listened_at >= current_date - interval '6 days'

WHERE scrobbles_raw.user_id = $1
GROUP BY w.period, w.isodow, w.date_val
ORDER BY w.date_val;
