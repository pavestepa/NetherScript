mod const_decl;
use crate::{
    semantics::{ast_mapper::decl::const_decl::const_decl_mapper, hir},
    syntax::ast,
};

pub fn decl_mapper(from: &ast::Decl) -> hir::Decl {
    match *from {
        ast::Decl::Const(val) => hir::Decl::Const(const_decl_mapper(val.unwrap())),
        ast::Decl::Enum(val) => {}
        ast::Decl::Export(val) => {}
        ast::Decl::Function(val) => {}
        ast::Decl::ImportDecl(val) => {}
        ast::Decl::IndexDecl(val) => {}
        ast::Decl::Struct(val) => {}
        ast::Decl::Type(val) => {}
        ast::Decl::Error => {
            panic!("ast::Decl is Error")
        }
    }
}
