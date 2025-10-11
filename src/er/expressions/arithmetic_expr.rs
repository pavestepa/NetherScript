use crate::er::{expressions::Expr, operators::ArithmeticOperator};

pub struct Arithmetic {
  pub left: ArithmeticExpr,
  pub operator: ArithmeticOperator,
  pub right: ArithmeticExpr
}
pub enum ArithmeticExpr {
  ArithmeticExpr(Box<ArithmeticExpr>),
  Expr(Box<Expr>)
}