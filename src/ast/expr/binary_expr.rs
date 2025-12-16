use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
  left: Box<Expr>,
  op: BinaryOperator,
  right: Box<Expr>,
}

impl BinaryExpr {
  pub fn new(left: Expr, op: BinaryOperator, right: Expr) -> Self {
    Self {
      left: Box::new(left),
      op: op,
      right: Box::new(right)
    }
  }
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