use crate::syntax::{
    ast::{ast::Ast, ReturnStmt},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_return_stmt(&mut self) -> Ast<ReturnStmt> {
        match self.current().kind {
            TokenKind::Semicolon => {
                self.parse(TokenKind::Semicolon);
                Ast::Parsed(ReturnStmt { arg: None })
            }
            _ => {
                let expr = self.parse_expr(0);
                self.parse(TokenKind::Semicolon);
                Ast::Parsed(ReturnStmt {
                    arg: Some(Box::from(expr)),
                })
            }
        }
    }
}
