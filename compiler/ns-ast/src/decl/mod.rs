mod class_decl;
mod const_decl;
mod enum_decl;
mod export_decl;
mod function_decl;
mod import_decl;
mod index_decl;
mod type_decl;

pub use class_decl::ClassDecl;
pub use const_decl::ConstDecl;
pub use enum_decl::EnumDecl;
pub use export_decl::ExportDecl;
pub use function_decl::FunctionDecl;
pub use import_decl::ImportDecl;
pub use index_decl::IndexDecl;
pub use type_decl::TypeDecl;

use crate::ast::Ast;

#[derive(Debug)]
pub enum Decl {
    Const(Ast<ConstDecl>),
    Enum(Ast<EnumDecl>),
    Export(Ast<ExportDecl>),
    Function(Ast<FunctionDecl>),
    ImportDecl(Ast<ImportDecl>),
    IndexDecl(Ast<IndexDecl>),
    Class(Ast<ClassDecl>),
    Type(Ast<TypeDecl>),
    Error,
}
