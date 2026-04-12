use crate::syntax::ast::{ast::Ast, Expr};

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    Equals,
    NotEquals,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub struct LogicalOp {
    pub left: Box<Ast<Expr>>,
    pub kind: LogicalOperator,
    pub right: Box<Ast<Expr>>,
}
