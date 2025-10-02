WITH months AS (SELECT TO_CHAR(month_start, 'Month') AS period,
                       DATE_PART('month', month_start) AS month_num,
                       month_start
                FROM (SELECT DATE_TRUNC('month', NOW()) - INTERVAL '1 month' * (12 - generate_series) AS month_start
                      FROM GENERATE_SERIES(1, 12)) AS month_series),
     monthly_listens AS (SELECT TO_CHAR(listened_at, 'Month')   AS period,
                                DATE_PART('month', listened_at) AS month_num,
                                DATE_TRUNC('month', listened_at) AS listen_month,
                                COUNT(*)                        AS listens
                         FROM scrobbles
                                  JOIN scrobbles_raw ON scrobbles.source_id = scrobbles_raw.id
                         WHERE listened_at >= DATE_TRUNC('month', NOW()) - INTERVAL '11 months'
                           AND listened_at < DATE_TRUNC('month', NOW()) + INTERVAL '1 month'
                         GROUP BY TO_CHAR(listened_at, 'Month'), DATE_PART('month', listened_at), DATE_TRUNC('month', listened_at))
SELECT m.period                as period,
       COALESCE(ml.listens, 0) AS listens
FROM months m
         LEFT JOIN monthly_listens ml ON m.month_start = ml.listen_month
ORDER BY m.month_start;
