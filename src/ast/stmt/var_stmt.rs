use crate::ast::{ast::Ast, shared::VarKind, TypedBinding};

#[derive(Debug)]
pub struct VarStmt {
    pub kind: VarKind,
    pub typed_binding: Ast<TypedBinding>,
}

impl VarStmt {
    pub fn new(kind: VarKind, typed_binding: Ast<TypedBinding>) -> Self {
        Self {
            kind: kind,
            typed_binding: typed_binding,
        }
    }
}
