SELECT Score,
       (
        SELECT COUNT(DISTINCT Score)
          FROM Scores
         WHERE Score > s.Score
       )+1 AS Rank
  FROM Scores s
 ORDER BY Score DESC
