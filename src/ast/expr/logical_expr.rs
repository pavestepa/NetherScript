use super::Expr;

#[derive(Debug)]
pub struct LogicalExpr {
  left: Box<Expr>,
  op: LogicalOperator,
  right: Box<Expr>,
}

#[derive(Debug)]
pub enum LogicalOperator {
  EqualTo
}