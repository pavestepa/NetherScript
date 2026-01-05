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
    pub fn parse_binary_expr(&mut self, left: Expr, token: Token) -> Result<BinaryExpr, String> {
        // TODO: parse_bimary_expr
        Ok(BinaryExpr::new(
            left,
            BinaryOperator::Plus,
            Expr::NumberLiteral(Atom::from("1")),
        ))
    }
}
