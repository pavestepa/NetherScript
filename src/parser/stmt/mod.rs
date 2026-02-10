use crate::{
    ast::Stmt,
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

mod binding_stmt;
mod stmts_block;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        println!("[STARTED] parse kind of Stmt");
        match self.current().kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Let => {
                    self.consume(TokenKind::Keyword(keyword));
                    Stmt::Binding(self.parse_binding_stmt_let())
                }
                Keyword::Var => {
                    self.consume(TokenKind::Keyword(keyword));
                    Stmt::Binding(self.parse_binding_stmt_const())
                }
                e => {
                    self.error(format!(
                        "Keyword {:?} can not be used for Stmt declaration",
                        e
                    ));
                    Stmt::Error
                }
            },
            e => {
                self.error(format!(
                    "Token {:?} is not keyword and not suitable for Stmt",
                    e
                ));
                Stmt::Error
            }
        }
    }
}
