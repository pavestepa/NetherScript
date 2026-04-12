use ns_ast::{LoopStmt, ast::Ast};

use crate::Parser;


impl Parser {
    pub fn parse_loop_stmt(&mut self) -> Ast<LoopStmt> {
        let body = self.parse_stmts_block();
        Ast::Parsed(LoopStmt::new(body))
    }
}
