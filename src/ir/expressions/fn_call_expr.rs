use crate::ir::expressions::Expr;

pub struct FnCallExpr {
  pub name: String,
  pub args: Vec<Expr>
}