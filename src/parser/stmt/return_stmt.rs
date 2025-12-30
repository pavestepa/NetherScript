use crate::{
    ast::{stmt::ReturnStmt, Expr},
    parser::Parser,
};

impl Parser {
    pub fn return_stmt(&mut self) -> Result<ReturnStmt, String> {
        // TODO: parse_return_stmt
        Ok(ReturnStmt::new(None))
    }
}
