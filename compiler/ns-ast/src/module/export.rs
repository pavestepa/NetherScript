use crate::{Decl, Ident};

#[derive(Debug)]
pub enum Export {
    Ident(Ident),
    Idents(Vec<Ident>),
    Decl(Decl),
}
