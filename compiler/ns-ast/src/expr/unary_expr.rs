use ns_lexer::TokenKind;

use crate::Expr;

#[derive(Debug, Clone)]
pub enum UnaryExpr {
    Minus(Box<Expr>),
    Plus(Box<Expr>),
    Not(Box<Expr>),
    BitNot(Box<Expr>),
    /// Unary `*expr` — dereference (Rust-style). Lexer token is `Star`; the parser
    /// chooses this only in unary position (distinct from binary `*`).
    Deref(Box<Expr>),
}

impl UnaryExpr {
    pub fn from(token: TokenKind, expr: Expr) -> Self {
        match token {
            TokenKind::Minus => UnaryExpr::Minus(Box::new(expr)),
            TokenKind::Plus => UnaryExpr::Plus(Box::new(expr)),
            TokenKind::Not => UnaryExpr::Not(Box::new(expr)),
            TokenKind::BitNot => UnaryExpr::BitNot(Box::new(expr)),
            TokenKind::Star => UnaryExpr::Deref(Box::new(expr)),
            e => panic!("Token {:?} is not suitable for unary expression", e),
        }
    }
}
