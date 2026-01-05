use crate::{
    ast::{
        expr::{BinaryExpr, BinaryOperator, LogicalExpr, LogicalOperator},
        Expr,
    },
    lexer::Token,
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_logical_expr(&mut self, left: Expr, op: Token) -> Result<LogicalExpr, String> {
        // TODO: parse_bimary_expr
        Ok(LogicalExpr::new(
            left,
            LogicalOperator::EqualTo,
            Expr::NumberLiteral(Atom::from("1")),
        ))
    }
}
