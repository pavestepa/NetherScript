use crate::Atom;

#[derive(Debug, Clone)]
pub enum TypeRef {
    Literal(Atom),
    Error,
}
