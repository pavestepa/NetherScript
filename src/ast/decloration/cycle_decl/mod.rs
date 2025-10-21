use crate::ast::{decloration::condition_decl::condition::Condition, statements::FnStatmntExpr};

pub struct CycleDecl {
    pub condition: Condition,
    pub body: Vec<FnStatmntExpr>,
}
