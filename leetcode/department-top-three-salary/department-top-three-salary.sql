SELECT d.Name AS Department,
       e.Name AS Employee,
       e.Salary AS Salary
  FROM Employee e
 INNER JOIN Department d
    ON d.Id = e.DepartmentId
 WHERE (
        SELECT COUNT(DISTINCT Salary)
          FROM Employee
         WHERE DepartmentId=e.DepartmentId
           AND Salary > e.Salary
       ) < 3
