use crate::ast::TypedBindingPtrn;

#[derive(Debug)]
pub struct StructDecl {
    pub fields: Vec<TypedBindingPtrn>,
}
