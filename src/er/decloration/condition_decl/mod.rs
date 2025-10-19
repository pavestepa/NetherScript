pub mod condition;
use condition::Condition;

use crate::er::expressions::FnCallExpr;

pub struct ConditionDecl {
    pub condition: Condition,
    pub body: FnCallExpr,
}
