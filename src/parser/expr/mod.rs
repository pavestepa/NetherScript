use crate::{
    Atom, ast::{CallExpr, Expr}, lexer::{Keyword, TokenKind}, parser::{Parser, ident}
};

impl Parser {
    pub fn parse_expr(&mut self) -> Expr {
        println!("[STARTED] parse kind of Expr");
        println!("{:?}", self.current().kind);
        self.parse_ref
        match self.current().kind {
            TokenKind::Ident(ident) => {},
            TokenKind::NumberLiteral(number) => {},
            TokenKind::StringLiteral(string) => {},
            TokenKind::BooleanLiteral(boolean) => {},
            TokenKind::LeftParen => {},
            e => {
                self.error(format!(
                    "Token {:?} is not keyword and not suitable for Expr",
                    self.current().kind
                ));
                Expr::Error
            }
        }
    }
}
