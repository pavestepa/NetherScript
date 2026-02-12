use crate::{
    ast::{ast::Ast, BreakStmt, Ident},
    lexer::{Keyword, Token, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_break_stmt(&mut self) -> Ast<BreakStmt> {
        match self.current().kind {
            TokenKind::Ident(ident) => {
                self.parse(TokenKind::Ident(ident));
                self.parse(TokenKind::Semicolon);
                Ast::Parsed(BreakStmt {
                    label: Some(Ident(ident)),
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
