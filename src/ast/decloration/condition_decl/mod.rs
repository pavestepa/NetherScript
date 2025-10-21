pub mod condition;
use condition::Condition;

use crate::ast::expressions::FnCallExpr;

pub struct ConditionDecl {
    pub condition: Condition,
    pub body: FnCallExpr,
}
