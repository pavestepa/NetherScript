use crate::{Ident, TypeNode, TypeParameter};

#[derive(Debug, Clone)]
pub struct TypeDecl {
    pub ident: Ident,
    pub type_parameters: Vec<TypeParameter>,
    pub assign: TypeNode,
}
