use std::thread::current;

use crate::{
    ast::Stmt,
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

mod assign_stmt;
mod binding_stmt;
mod break_stmt;
mod expr_stmt;
mod if_stmt;
mod loop_stmt;
mod return_stmt;
mod stmts_block;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        println!("[STARTED] parse kind of Stmt");
        match self.current().kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Let => {
                    self.parse(TokenKind::Keyword(keyword));
                    Stmt::Binding(self.parse_binding_stmt_let())
                }
                Keyword::Var => {
                    self.parse(TokenKind::Keyword(keyword));
                    Stmt::Binding(self.parse_binding_stmt_var())
                }
                Keyword::If => {
                    self.parse(TokenKind::Keyword(keyword));
                    Stmt::If(self.parse_if_stmt())
                }
                Keyword::Break => {
                    self.parse(TokenKind::Keyword(keyword));
                    Stmt::Break(self.parse_break_stmt())
                }
                Keyword::Loop => {
                    self.parse(TokenKind::Keyword(keyword));
                    Stmt::Loop(self.parse_loop_stmt())
                }
                Keyword::Return => {
                    self.parse(TokenKind::Keyword(keyword));
                    Stmt::Return(self.parse_return_stmt())
                }
                e => {
                    self.error(format!(
                        "Keyword {:?} can not be used for Stmt declaration",
                        e
                    ));
                    Stmt::Error
                }
            },
            TokenKind::Ident(ident) => {
                let expr_res = self.try_parse_expr_stmt();
                if expr_res.is_some() {}

                self.parse(TokenKind::Ident(ident));
                match self.current().kind {
                    TokenKind::Assign => {
                        self.parse(TokenKind::Assign);
                        Stmt::Assign(self.parse_assign_stmt(ident))
                    }
                    e => {
                        self.error(format!(
                            "Token {:?} is not keyword and not suitable for function call or assign operation",
                            e
                        ));
                        Stmt::Error
                    }
                }
            }
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
