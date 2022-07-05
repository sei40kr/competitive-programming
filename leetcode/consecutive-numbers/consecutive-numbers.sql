SELECT DISTINCT l3.Num AS ConsecutiveNums
  FROM Logs l3
 INNER JOIN Logs l2
    ON l2.Id = l3.Id-1
 INNER JOIN Logs l1
    ON l1.Id = l3.Id-2
 WHERE l1.Num = l2.Num
   AND l2.Num = l3.Num
