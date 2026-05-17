mod class_decl;
mod const_decl;
mod enum_decl;
mod function_decl;
mod interface_decl;
mod type_decl;
mod type_modifier_decl;

pub use class_decl::ClassDecl;
pub use const_decl::ConstDecl;
pub use enum_decl::EnumDecl;
pub use function_decl::FunctionDecl;
pub use interface_decl::InterfaceDecl;
pub use type_decl::TypeDecl;
pub use type_modifier_decl::{ExtendsDecl, ImplementsDecl, TypeModifierDecl};

#[derive(Debug, Clone)]
pub struct ErrorDecl {
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Class(ClassDecl),
    Const(ConstDecl),
    Error(ErrorDecl),
    Enum(EnumDecl),
    Function(FunctionDecl),
    Interface(InterfaceDecl),
    Type(TypeDecl),
    TypeModifier(TypeModifierDecl),
}
