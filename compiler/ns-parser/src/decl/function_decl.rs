use ns_ast::FunctionDecl;

use crate::Parser;

impl Parser {
    pub fn parse_function_decl(&mut self) -> FunctionDecl {
        let signature = self.parse_callable_signature(false);
        let body = self.parse_stmts_block();
        FunctionDecl { signature, body }
    }
}
