use ns_ast::{BreakStmt, Ident, ast::Ast};
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_break_stmt(&mut self) -> Ast<BreakStmt> {
        match self.current().kind {
            TokenKind::Ident(ident) => {
                self.parse(TokenKind::Ident(ident));
                self.parse(TokenKind::Semicolon);
                Ast::Parsed(BreakStmt {
                    label: Some(Ident::new(ident)),
                })
            }
            TokenKind::Semicolon => {
                self.parse(TokenKind::Semicolon);
                Ast::Parsed(BreakStmt { label: None })
            }
            e => {
                let err = format!("expected ';', or ident of break, found {:?}", e);
                self.error(err.clone());
                Ast::Error(err)
            }
        }
    }
}
