use crate::semantics::hir::decls::Decl;

mod binding;
mod data_type;
mod decls;
mod enum_member;
mod exprs;
mod ref_kind;
mod stmts;

pub struct Module {
    pub decls: Vec<Decl>,
}
