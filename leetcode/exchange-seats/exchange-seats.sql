SELECT CONVERT(@id:=@id+1,SIGNED) AS id,
       student
  FROM seat,
       (
        SELECT @id:=0
       ) _
 ORDER BY id+(-0.5+id%2)*2
