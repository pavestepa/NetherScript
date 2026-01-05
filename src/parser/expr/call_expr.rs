use crate::{
    ast::{expr::CallExpr, Expr},
    parser::Parser,
};

impl Parser {
    pub fn parse_call_expr(&mut self, left: Expr) -> Result<CallExpr, String> {
        // TODO: parse args
        Ok(CallExpr::new(left, vec![]))
    }
}
