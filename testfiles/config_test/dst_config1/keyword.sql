SELECT
	CASE
		WHEN
			A	=	1
		THEN
			'one'
		ELSE
			'other'
	END
		AS	GRADE
FROM
	STUDENT	STD
WHERE
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
FROM
	PRODUCTS
WHERE
	OBSOLETION_DATE	=	'today'
RETURNING
	*
;
INSERT
INTO
	DISTRIBUTORS
(
	DID
,	DNAME
) VALUES (
	DEFAULT
,	'XYZ Widgets'
)
RETURNING
	DID
;
