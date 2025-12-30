use crate::Atom;

#[derive(Debug)]
pub struct BreakStmt {
    pub label: Option<Atom>,
}

impl BreakStmt {
    pub fn new(label: Option<Atom>) -> Self {
        Self { label }
    }
}
