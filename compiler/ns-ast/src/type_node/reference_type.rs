use crate::{RefKind, type_node::TypeNode};

#[derive(Debug, Clone)]
pub struct ReferenceType {
    pub kind: RefKind,
    pub argument: Box<TypeNode>,
}
impl ReferenceType {
    pub fn new(kind: RefKind, argument: Box<TypeNode>) -> Self {
        Self { kind, argument }
    }
}
