use crate::{Ident, TypeNode};

#[derive(Debug)]
pub struct TypeDecl {
    pub ident: Ident,
    pub assign: TypeNode,
}
