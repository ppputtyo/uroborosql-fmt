UPDATE TABLE1 TBL1-- テーブル1
SET TBL1.COLUMN2 = TBL2.COLUMNX -- カラム2
, TBL1.COLUMN3 = 100 -- カラム3
-- コメント
from
			TABLE2 TBL2-- テーブル2
WHERE TBL1.COLUMN1	=	10 AND TBL1.COLUMN4 = TBL2.COLUMNY
    