use crate::Ident;

#[derive(Debug, Clone)]
pub struct BreakStmt {
    pub label: Option<Ident>,
}

impl BreakStmt {
    pub fn new(label: Option<Ident>) -> Self {
        Self { label }
    }
}
