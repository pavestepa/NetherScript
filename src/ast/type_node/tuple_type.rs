use crate::ast::type_node::TypeNode;

#[derive(Debug, Clone)]
pub struct TupleType {
    pub body: Vec<TypeNode>,
}
