select
  *
from
  employee  emp
where
emp.birth_date  between  /*birth_date_from*/'1990-01-01'  and  /*birth_date_to*/'1999-12-31'

/*%if SF.isNotEmpty(birth_date_from) and SF.isNotEmpty(birth_date_to)*/ -- domaはelifではなくelseif

limit 
all
OFFSET 10
/*%elseif SF.isNotEmpty(birth_date_from)*/
limit 
all
OFFSET 5
/*%end*/
;