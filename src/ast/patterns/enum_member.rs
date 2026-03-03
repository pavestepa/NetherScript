use crate::ast::{Ident, TypeNode};

#[derive(Debug)]
pub struct EnumMember {
    pub ident: Ident,
    pub contains: Option<TypeNode>,
}
