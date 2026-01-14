use crate::{ast::BreakStmt, parser::Parser};

impl Parser {
    pub fn parse_break_stmt(&mut self) -> Result<BreakStmt, String> {
        // TODO: parse_break_stmt
        return Ok(BreakStmt::new(None));
    }
}
