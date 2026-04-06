pub mod binding;
pub mod data_type;
pub mod decls;
pub mod enum_member;
pub mod exprs;
pub mod ref_kind;
pub mod stmts;

pub use decls::Decl;
pub struct Module {
    pub decls: Vec<Decl>,
}
