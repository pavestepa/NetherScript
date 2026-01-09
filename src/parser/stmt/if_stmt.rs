use crate::{
    ast::{
        stmt::{BlockStmt, IfStmt},
        Expr, Stmt,
    },
    parser::Parser,
};

impl Parser {
    pub fn parse_if_stmt(&mut self) -> Result<IfStmt, String> {
        // TODO: parse_if_stmt
        Ok(IfStmt::new(Expr::Empty, BlockStmt::new(), None))
    }
}
