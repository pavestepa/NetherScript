use crate::Atom;

#[derive(Debug)]
pub struct BreakStmt {
    pub label: Option<Atom>,
}
