use crate::{Callable, StmtsBlock, TypeNode};

#[derive(Debug, Clone)]
pub enum TypeModifierDecl {
    Extends(ExtendsDecl),
    Implements(ImplementsDecl),
}

/// Adds methods to a named type outside that type’s primary definition.
///
/// Name resolution and dispatch treat these methods as part of the target type.
#[derive(Debug, Clone)]
pub struct ExtendsDecl {
    /// Type that receives the methods declared in this block.
    pub type_identifier: TypeNode,
    pub methods: Vec<(Callable, StmtsBlock)>,
}

#[derive(Debug, Clone)]
pub struct ImplementsDecl {
    /// Type that implements the interfaces declared in this block.
    pub type_identifier: TypeNode,
    /// Interfaces/types that this type implements.
    pub interfaces: Vec<TypeNode>,
    /// Methods that this type implements.
    pub methods: Vec<(Callable, StmtsBlock)>,
}
