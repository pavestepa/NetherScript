use crate::{
    ast::decloration::{condition_decl::ConditionDecl, VarDecl},
    ir::cycle_decl::CycleDecl,
};

pub enum FnStatmntExpr {
    VarDecl(VarDecl),
    ConditionDecl(ConditionDecl),
    CycleDecl(CycleDecl),
}
