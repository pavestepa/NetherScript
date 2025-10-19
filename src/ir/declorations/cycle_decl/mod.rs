use crate::ir::{condition_decl::Condition, FnCallExpr};

pub struct CycleDecl {
    pub condition: Condition,
    pub body: FnCallExpr,
}
