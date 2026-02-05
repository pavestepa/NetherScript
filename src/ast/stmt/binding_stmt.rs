use crate::ast::{ast::Ast, Binding, LetOrVar};

#[derive(Debug)]
pub struct BindingStmt {
    pub kind: LetOrVar,
    pub typed_binding: Ast<Binding>,
}

impl BindingStmt {
    pub fn new(kind: LetOrVar, typed_binding: Ast<Binding>) -> Self {
        Self {
            kind: kind,
            typed_binding: typed_binding,
        }
    }
}
