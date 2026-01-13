mod const_decl;
mod enum_decl;
mod function_decl;
mod implement_decl;
mod import_decl;
mod index_decl;
mod struct_decl;
mod type_decl;

pub use const_decl::ConstDecl;
pub use enum_decl::EnumDecl;
pub use function_decl::FunctionDecl;
pub use implement_decl::ImplementDecl;
pub use import_decl::ImportDecl;
pub use index_decl::IndexDecl;
pub use struct_decl::StructDecl;
pub use type_decl::TypeDecl;

#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    Function(FunctionDecl),
    Implement(ImplementDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Type(TypeDecl),
    IndexDecl(IndexDecl),
    ImportDecl(ImportDecl),
}
