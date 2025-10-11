use crate::{er::types::HasType, ir::{declorations::var_decl::VarDecl, FnCallExpr}};

pub enum FnExpr {
  VarDecl(VarDecl),
  ConditionExpr,
  CycleExpr,
  FnCall(FnCallExpr),
  FnReturn(HasType),
}