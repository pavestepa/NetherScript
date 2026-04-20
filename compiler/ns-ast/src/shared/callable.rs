use crate::{Ident, This, TypeNode, TypedBinding};

#[derive(Debug)]
pub struct Callable {
    pub ident: Ident,
    pub this: This,
    pub arguments: Vec<TypedBinding>,
    pub return_type: TypeNode,
}
