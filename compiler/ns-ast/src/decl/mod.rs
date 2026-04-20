mod class_decl;
mod const_decl;
mod enum_decl;
mod function_decl;
mod interface_decl;
mod type_decl;

pub use class_decl::ClassDecl;
pub use const_decl::ConstDecl;
pub use enum_decl::EnumDecl;
pub use function_decl::FunctionDecl;
pub use interface_decl::InterfaceDecl;
pub use type_decl::TypeDecl;

#[derive(Debug)]
pub enum Decl {
    Class(ClassDecl),
    Const(ConstDecl),
    Enum(EnumDecl),
    Function(FunctionDecl),
    Interface(InterfaceDecl),
    Type(TypeDecl),
}
