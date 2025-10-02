WITH time_series AS (
    SELECT generate_series(
        date_trunc('day', now()),
        date_trunc('day', now()) + INTERVAL '23 hour',
        '1 hour'
    ) AS period
)

SELECT
    extract(HOUR FROM ts.period)::TEXT AS period,
    coalesce(count(sr.listened_at), 0) AS listens
FROM time_series AS ts
    LEFT JOIN scrobbles_raw AS sr
        ON
            date_trunc('hour', sr.listened_at) = ts.period
            AND sr.user_id = $1
            AND sr.listened_at >= date_trunc('day', now())
GROUP BY ts.period
ORDER BY ts.period;
