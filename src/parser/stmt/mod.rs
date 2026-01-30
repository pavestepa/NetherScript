use crate::{
    ast::Stmt,
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

mod block_stmt;
mod var_stmt;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        match self.token() {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Let => {
                    self.next();
                    Stmt::Var(self.parse_var_stmt_let())
                }
                Keyword::Const => {
                    self.next();
                    Stmt::Var(self.parse_var_stmt_const())
                }
                e => {
                    self.go_to_next_stmt();
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
                self.go_to_next_stmt();
                Stmt::Error
            }
        }
    }
    pub fn go_to_next_stmt(&mut self) {
        loop {
            if self.peek().is_some() {
                match self.token() {
                    TokenKind::Keyword(keyword) => match keyword {
                        Keyword::Const | Keyword::Let => {
                            break;
                        }
                        _ => self.position += 1,
                    },
                    TokenKind::RightBrace => {
                        break;
                    }
                    _ => self.position += 1,
                }
            } else {
                break;
            }
        }
    }
}
