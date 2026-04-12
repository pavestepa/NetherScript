use crate::{Ident, TypeNode};

#[derive(Debug)]
pub struct EnumMember {
    pub ident: Ident,
    pub contains: Option<TypeNode>,
}
