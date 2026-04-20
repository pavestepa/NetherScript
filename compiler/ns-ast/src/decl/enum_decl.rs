use crate::{EnumMember, Ident};

#[derive(Debug)]
pub struct EnumDecl {
    pub ident: Ident,
    pub members: Vec<EnumMember>,
}
