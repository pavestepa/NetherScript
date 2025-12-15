use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
  left: Box<Expr>,
  op: BinaryOperator,
  right: Box<Expr>,
}

#[derive(Debug)]
pub enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
  Equals,
  NotEquals,
}