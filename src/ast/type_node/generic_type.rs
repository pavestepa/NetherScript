use crate::ast::{ast::Ast, type_node::TypeNode, Ident};

#[derive(Debug, Clone)]
pub struct GenericType {
    pub ident: Ident,
    pub arguments: Option<Vec<Ast<TypeNode>>>,
}
impl GenericType {
    pub fn new(ident: Ident, arguments: Option<Vec<Ast<TypeNode>>>) -> Self {
        Self { ident, arguments }
    }
}
