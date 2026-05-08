use crate::Ident;

#[derive(Debug)]
pub struct Import {
    pub ident: Ident,
    pub from: Vec<Ident>,
}
