use crate::ast::{Ident, TypeRef};

#[derive(Debug)]
pub struct EnumMember {
    pub ident: Ident,
    pub contains: Option<TypeRef>,
}
