use crate::syntax::ast::{ast::Ast, Binding, Expr, LetOrVar};

#[derive(Debug)]
pub struct BindingStmt {
    pub kind: LetOrVar,
    pub typed_binding: Ast<Binding>,
    pub assign: Option<Box<Ast<Expr>>>,
}

impl BindingStmt {
    pub fn new(
        kind: LetOrVar,
        typed_binding: Ast<Binding>,
        assign: Option<Box<Ast<Expr>>>,
    ) -> Self {
        Self {
            kind,
            typed_binding,
            assign,
        }
    }
}
