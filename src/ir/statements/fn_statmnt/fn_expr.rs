use crate::{
    ast::Typ,
    ir::{declorations::var_decl::VarDecl, FnCallExpr},
};

pub enum FnExpr {
    VarDecl(VarDecl),
    ConditionExpr,
    CycleExpr,
    FnCall(FnCallExpr),
    FnReturn(Typ),
}
