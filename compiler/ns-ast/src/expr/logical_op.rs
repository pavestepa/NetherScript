use crate::Expr;

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
    pub left: Box<Expr>,
    pub kind: LogicalOperator,
    pub right: Box<Expr>,
}
