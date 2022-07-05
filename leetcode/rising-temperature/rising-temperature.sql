SELECT t.Id
  FROM Weather t
 INNER JOIN Weather y
    ON y.RecordDate = t.RecordDate - interval 1 DAY
 WHERE y.Temperature < t.Temperature
