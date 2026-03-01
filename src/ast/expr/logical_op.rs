use crate::ast::Expr;

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
    left: Box<Expr>,
    kind: LogicalOperator,
    right: Box<Expr>,
}
