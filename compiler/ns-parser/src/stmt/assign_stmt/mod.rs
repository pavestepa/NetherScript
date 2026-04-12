use ns_ast::{AssignStmt, Ident, ast::Ast};
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_assign_stmt(&mut self, ident: Ident) -> Ast<AssignStmt> {
        let assign = self.parse_expr(0);
        self.parse(TokenKind::Semicolon);
        Ast::Parsed(AssignStmt {
            ident: ident,
            assign: Box::from(assign),
        })
    }
}
