select
	CASE
		when
			A	=	1
		THEN
			'one'
		else
			'other'
	END
		AS	GRADE
from
	STUDENT	STD
where
	GRADE	BETWEEN		/*start1*/60	AND	/*end1*/100
AND	GRADE	NOT BETWEEN	/*start2*/70	AND	/*end2*/80
;
UPDATE
	WEATHER
SET
	(TEMP_LO, TEMP_HI, PRCP)	=	(TEMP_LO	+	1, TEMP_LO	+	15, DEFAULT)
WHERE
	CITY	=	'San Francisco'
;
DELETE
from
	PRODUCTS
WHERE
	OBSOLETION_DATE	=	'today'
RETURNING
	*
;
INSERT
into
	DISTRIBUTORS
(
	DID
,	DNAME
) VALUES (
	default
,	'XYZ Widgets'
)
RETURNING
	DID
;
