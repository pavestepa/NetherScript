pub use crate::semantics::hir::decls::{
    export::ExportDecl, import::ImportDecl, r#const::ConstDecl, r#enum::EnumDecl, r#fn::FnDecl,
    r#struct::StructDecl, r#type::TypeDecl,
};

mod r#const;
mod r#enum;
mod export;
mod r#fn;
mod import;
mod r#struct;
mod r#type;

pub enum Decl {
    Const(ConstDecl),
    Enum(EnumDecl),
    Export(ExportDecl),
    Fn(FnDecl),
    Import(ImportDecl),
    Struct(StructDecl),
    Type(TypeDecl),
}
