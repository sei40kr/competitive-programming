SELECT Request_at AS Day,
       ROUND(SUM(CASE
                     WHEN t.Status <> 'completed'
                          AND u.Banned = 'No' THEN 1
                     ELSE 0
                 END) / COUNT(*), 2) AS "Cancellation Rate"
FROM Trips AS t
INNER JOIN Users AS u ON t.Client_Id = u.Users_Id
WHERE t.Request_at >= '2013-10-01'
  AND t.Request_at <= '2013-10-03'
  AND u.Banned = 'No'
GROUP BY Request_at
