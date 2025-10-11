use crate::ir::expressions::Expr;

pub struct ObjectExpr {
  pub struct_type_name: String,
  pub args: Vec<Expr>
}