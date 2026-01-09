use crate::{
    ast::{
        expr::{BinaryExpr, BinaryOperator},
        Expr,
    },
    lexer::Token,
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_binary_expr(
        &mut self,
        left: &mut Expr,
        op: BinaryOperator,
    ) -> Result<BinaryExpr, String> {
        // TODO: parse_bimary_expr
        Ok(BinaryExpr::new(
            left.clone(),
            BinaryOperator::Plus,
            Expr::NumberLiteral(Atom::from("1")),
        ))
    }
}
