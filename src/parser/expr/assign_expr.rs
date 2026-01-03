use crate::{
    ast::{
        expr::{AssignExpr, AssignOperator},
        Expr,
    },
    atom,
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_assign_expr(&mut self) -> Result<AssignExpr, String> {
        // TODO: parse_assign_expression
        Ok(AssignExpr::new(
            Expr::Boolean(true),
            AssignOperator::Assign,
            Expr::NumberLiteral(Atom::from("1")),
        ))
    }
}
