SELECT
	STD.GRADE	AS	GRADE
FROM
	STUDENT	STD
WHERE
	GRADE	BETWEEN		/*start1*/60	AND	/*end1*/100	-- between
AND	GRADE	NOT BETWEEN	/*start2*/70	AND	/*end2*/80	-- not between
