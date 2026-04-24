use ns_lexer::TokenKind;

use crate::Expr;

#[derive(Debug, Clone)]
pub enum UnaryExpr {
    Minus(Box<Expr>),
    Plus(Box<Expr>),
    Not(Box<Expr>),
    BitNot(Box<Expr>),
    /// Indirection: reads the value behind one level of pointer or reference-like storage on the operand.
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
