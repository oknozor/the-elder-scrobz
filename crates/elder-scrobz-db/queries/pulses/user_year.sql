WITH months AS (SELECT TO_CHAR(TO_DATE(m::text, 'MM'), 'Month') AS period,
                       m                                        AS month_num
                FROM GENERATE_SERIES(1, 12) AS m),
     monthly_listens AS (SELECT TO_CHAR(listened_at, 'Month')   AS period,
                                DATE_PART('month', listened_at) AS month_num,
                                COUNT(*)                        AS listens
                         FROM scrobbles
                                  JOIN scrobbles_raw ON scrobbles.source_id = scrobbles_raw.id
                         WHERE EXTRACT(YEAR FROM listened_at) = EXTRACT(YEAR FROM NOW()) AND scrobbles_raw.user_id = $1
                         GROUP BY TO_CHAR(listened_at, 'Month'), DATE_PART('month', listened_at))
SELECT m.period                as period,
       COALESCE(ml.listens, 0) AS listens
FROM months m
         LEFT JOIN monthly_listens ml ON m.month_num = ml.month_num
ORDER BY m.month_num;
