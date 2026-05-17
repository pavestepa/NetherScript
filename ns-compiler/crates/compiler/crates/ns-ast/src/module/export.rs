use crate::Ident;

#[derive(Debug)]
pub enum Export {
    Ident(Ident),
    Idents(Vec<Ident>),
}
