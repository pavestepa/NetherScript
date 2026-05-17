use ns_atom::Atom;

#[derive(Debug, Clone)]
pub struct Ident(Atom);

impl Ident {
    pub fn new(val: Atom) -> Self {
        Self(val)
    }
    pub fn into_simple(self) -> Atom {
        self.0
    }
}
