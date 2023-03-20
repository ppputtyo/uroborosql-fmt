use tree_sitter::TreeCursor;

use crate::{
    cst::*,
    formatter::{ensure_kind, Formatter},
};

use super::is_comp_op;

impl Formatter {
    pub(crate) fn format_binary_expr(
        &mut self,
        cursor: &mut TreeCursor,
        src: &str,
    ) -> Result<Expr, UroboroSQLFmtError> {
        // cursor -> binary_expression

        cursor.goto_first_child();
        // cursor -> _expression

        // 左辺
        let lhs_expr = self.format_expr(cursor, src)?;

        cursor.goto_next_sibling();
        // cursor -> op (e.g., "+", "-", "=", ...)

        // 演算子
        let op_node = cursor.node();
        let op_str = op_node.utf8_text(src.as_ref()).unwrap();

        cursor.goto_next_sibling();
        // cursor -> _expression

        // 右辺
        let rhs_expr = self.format_expr(cursor, src)?;

        // cursorを戻しておく
        cursor.goto_parent();
        ensure_kind(cursor, "binary_expression")?;

        if is_comp_op(op_str) {
            // 比較演算子ならばそろえる必要があるため、AlignedExprとする
            let mut aligned = AlignedExpr::new(lhs_expr, false);
            aligned.add_rhs(op_str, rhs_expr);

            Ok(Expr::Aligned(Box::new(aligned)))
        } else {
            // 比較演算子でない(算術演算等)ならば、ExprSeq に

            // 実装の都合上、演算子を PrimaryExpr として扱う
            let op_prim = PrimaryExpr::with_node(op_node, src);
            let op_expr = Expr::Primary(Box::new(op_prim));

            let bin_expr = ExprSeq::new(&vec![lhs_expr, op_expr, rhs_expr]);
            Ok(Expr::ExprSeq(Box::new(bin_expr)))
        }
    }
}
