mod class_decl;
pub use class_decl::ClassDecl;
mod fn_decl;
pub use fn_decl::{FnDecl, fn_arg::FnArg};
mod global_const_decl;
pub use global_const_decl::GlobalConstDecl;
mod type_decl;
pub use type_decl::TypeDecl;

#[derive(Debug)]
pub enum Decl {
    ClassDecl(ClassDecl),
    FnDecl(FnDecl),
    GlobalConstDecl(GlobalConstDecl),
    TypeDecl(TypeDecl),
}
