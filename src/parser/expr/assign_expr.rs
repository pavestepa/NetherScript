use crate::{
    ast::{
        expr::{AssignExpr, AssignOperator},
        Expr,
    },
    lexer::Token,
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_assign_expr(&mut self, left: Expr) -> Result<AssignExpr, String> {
        // TODO: parse_assign_expression
        Ok(AssignExpr::new(
            left,
            AssignOperator::Assign,
            Expr::NumberLiteral(Atom::from("1")),
        ))
    }
}
