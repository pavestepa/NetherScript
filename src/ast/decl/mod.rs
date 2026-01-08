mod class_decl;
mod const_decl;
mod function_decl;
mod implement_decl;
mod struct_decl;
mod trait_decl;

pub use class_decl::ClassDecl;
pub use const_decl::ConstDecl;
pub use function_decl::{FnArg, FunctionDecl};
pub use implement_decl::ImplementDecl;
pub use struct_decl::StructDecl;
pub use trait_decl::TraitDecl;

#[derive(Debug)]
pub enum Decl {
    Class(ClassDecl),
    Const(ConstDecl),
    Function(FunctionDecl),
    Implement(ImplementDecl),
    Struct(StructDecl),
    Trait(TraitDecl),
}
