use crate::Atom;

#[derive(Debug)]
pub struct ClassDecl {
    is_pub: bool,
    ident: Atom,
    extends: Option<Atom>,
    implements: Option<Vec<Atom>>,
    fields: Vec<Atom>, //TODO
}

impl ClassDecl {
    pub fn new(
        is_pub: bool,
        ident: Atom,
        extends: Option<Atom>,
        implements: Option<Vec<Atom>>,
        fields: Vec<Atom>,
    ) -> Self {
        Self {
            is_pub,
            ident,
            extends,
            implements,
            fields,
        }
    }
}
