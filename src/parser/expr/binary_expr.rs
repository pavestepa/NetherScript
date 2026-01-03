use crate::{
    ast::{
        expr::{BinaryExpr, BinaryOperator},
        Expr,
    },
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_binary_expr(&mut self) -> Result<BinaryExpr, String> {
        // TODO: parse_bimary_expr
        Ok(BinaryExpr::new(
            Expr::NumberLiteral(Atom::from("1")),
            BinaryOperator::Plus,
            Expr::NumberLiteral(Atom::from("1")),
        ))
    }
}
