SELECT
	*
FROM
	DEPARTMENT
WHERE
	ID		IN	(
		SELECT
			DEPARTMENT_ID
		FROM
			USER
		WHERE
			ADDRESS	=	'TOKYO'
	)
AND	TEST	=	TEST
