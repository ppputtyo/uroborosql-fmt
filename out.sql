SELECT
	A
,	(
		SELECT
			Z	--test
		FROM
			TAB2
	)
FROM
	LONGLONGTABLE	AS	L
,	(
		SELECT
			B
		,	C
		FROM
			TAB1
	)				AS	BC

