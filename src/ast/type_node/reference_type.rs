use crate::ast::{ast::Ast, type_node::TypeNode, RefKind};

#[derive(Debug, Clone)]
pub struct ReferenceType {
    pub kind: RefKind,
    pub argument: Box<Ast<TypeNode>>,
}
impl ReferenceType {
    pub fn new(kind: RefKind, argument: Box<Ast<TypeNode>>) -> Self {
        Self { kind, argument }
    }
}
