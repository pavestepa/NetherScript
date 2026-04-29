use ns_ast::LoopStmt;

use crate::Parser;

impl Parser {
    pub fn parse_loop_stmt(&mut self) -> LoopStmt {
        LoopStmt::new(self.parse_stmts_block())
    }
}
