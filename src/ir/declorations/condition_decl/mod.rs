mod condition; pub use condition::Condition;

use crate::ir::FnCallExpr;

pub struct ConditionDecl {
  pub condition: Condition,
  pub body: FnCallExpr
}