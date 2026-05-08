use crate::{Ident, TypeNode};

#[derive(Debug, Clone)]
pub struct EnumMember {
    pub ident: Ident,
    pub contains: Option<TypeNode>,
}
