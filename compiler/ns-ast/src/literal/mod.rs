use ns_atom::Atom;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(Atom),
    String(Atom),
    Boolean(Atom),
}
