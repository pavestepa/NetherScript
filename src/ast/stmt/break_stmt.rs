use crate::ast::Ident;

#[derive(Debug)]
pub struct BreakStmt {
    pub label: Option<Ident>,
}

impl BreakStmt {
    pub fn new(label: Option<Ident>) -> Self {
        Self { label }
    }
}
