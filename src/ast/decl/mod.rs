mod class_decl;
pub use class_decl::ClassDecl;
mod enum_decl;
pub use enum_decl::EnumDecl;
mod export_decl;
pub use export_decl::ExportDecl;
mod fn_decl;
pub use fn_decl::FnDecl;
mod global_const_decl;
pub use global_const_decl::GlobalConstDecl;
mod import_decl;
pub use import_decl::ImportDecl;
mod interface_decl;
pub use interface_decl::InterfaceDecl;
mod struct_decl;
pub use struct_decl::StructDecl;
mod type_decl;
pub use type_decl::TypeDecl;

#[derive(Debug)]
pub enum Decl {
    ClassDecl(ClassDecl),
    EnumDecl(EnumDecl),
    ExportDecl(ExportDecl),
    FnDecl(FnDecl),
    GlobalConstDecl(GlobalConstDecl),
    ImportDecl(ImportDecl),
    InterfaceDecl(InterfaceDecl),
    StructDecl(StructDecl),
    TypeDecl(TypeDecl),
}
