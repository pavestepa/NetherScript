use crate::{Ident, TypeNode};

#[derive(Debug)]
pub struct TypeDecl {
    ident: Ident,
    type_value: TypeNode,
}
