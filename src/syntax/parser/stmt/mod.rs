use crate::syntax::{
    ast::{Ident, Stmt},
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
                if self.peek(1).kind == TokenKind::Assign {
                    self.parse(self.current().kind); // ident
                    self.parse(self.current().kind); // assign
                    return Stmt::Assign(self.parse_assign_stmt(Ident::new(ident)));
                } else {
                    return Stmt::Expr(self.parse_expr_stmt());
                }
            }
            TokenKind::LeftParen
            | TokenKind::NumberLiteral(_)
            | TokenKind::StringLiteral(_)
            | TokenKind::BooleanLiteral(_) => Stmt::Expr(self.parse_expr_stmt()),
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
