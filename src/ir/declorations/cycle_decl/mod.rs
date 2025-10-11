use swc_ecma_ast::FnExpr;

use crate::ir::condition_decl::Condition;


pub struct CycleDecl {
  pub condition: Condition,
  pub body: FnExpr
}