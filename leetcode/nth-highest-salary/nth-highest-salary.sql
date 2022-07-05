CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  RETURN (
    SELECT Salary
    FROM
      (SELECT Salary
       FROM Employee AS e1
       WHERE
           (SELECT COUNT(DISTINCT(e2.Salary))
            FROM Employee AS e2
            WHERE e2.Salary > e1.Salary) = N - 1) AS t
    UNION ALL
    SELECT NULL
    FROM dual
    LIMIT 1
  );
END

