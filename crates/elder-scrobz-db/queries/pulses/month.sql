WITH all_days AS (
    SELECT generate_series(
        current_date - interval '29 days',
        current_date,
        interval '1 day'
    )::date AS day
)

SELECT
    extract(DAY FROM all_days.day)::text
    || '/'
    || lpad(extract(MONTH FROM all_days.day)::text, 2, '0') AS period,
    coalesce(count(scrobbles_raw.listened_at), 0) AS listens
FROM all_days
    LEFT JOIN scrobbles_raw
        ON date_trunc('day', scrobbles_raw.listened_at) = all_days.day
WHERE
    scrobbles_raw.listened_at >= current_date - interval '29 days'
    OR scrobbles_raw.listened_at IS NULL
GROUP BY all_days.day
ORDER BY all_days.day;
