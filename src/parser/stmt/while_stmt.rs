use crate::{
    ast::{
        stmt::{BlockStmt, WhileStmt},
        Expr,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_while_stmt() -> Result<WhileStmt, String> {
        // TODO: parse_while_stmt
        Ok(WhileStmt::new(Expr::Boolean(true), BlockStmt::new()))
    }
}
