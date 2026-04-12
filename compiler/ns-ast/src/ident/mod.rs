use ns_atom::Atom;

#[derive(Debug, Clone)]
pub struct Ident {
    pub dol: bool, //* "$" prefix
    pub val: Atom
}

impl Ident {
    pub fn new(val: Atom) -> Self {
        if let Some(first) = val.as_str().chars().next() {
            return match first {
                '$' => Self {
                    dol: true,
                    val
                },
                _ => Self {
                    dol: false,
                    val
                }
            };
        }
        Self {
            dol: false,
            val
        }
    }
    pub fn into_simple(self) -> Atom {
        self.val
    }
}
