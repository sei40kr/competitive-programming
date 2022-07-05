SELECT d.Name AS Department,
       e.Name AS Employee,
       e.Salary
FROM Employee AS e
INNER JOIN
  (SELECT DepartmentId,
          MAX(Salary) AS MaxSalary
   FROM Employee
   GROUP BY DepartmentId) AS t ON e.DepartmentId = t.DepartmentId
AND e.Salary = t.MaxSalary
INNER JOIN Department AS d ON e.DepartmentId = d.Id
