use crate::{
    ast::{expr::MemberExpr, Expr},
    parser::Parser,
};

impl Parser {
    pub fn parse_member_expr(&mut self, left: Expr) -> Result<MemberExpr, String> {
        // TODO: member expr parse
        Ok(MemberExpr::new(left, Expr::Boolean(true)))
    }
}
