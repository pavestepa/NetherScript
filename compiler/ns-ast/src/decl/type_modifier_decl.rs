use crate::{Callable, Ident, StmtsBlock, TypeParameter};

#[derive(Debug)]
pub enum TypeModifierDecl {
    Extends(ExtendsDecl),
    Implements(ImplementsDecl),
}

/// Adds methods to a named type outside that type’s primary definition.
///
/// Name resolution and dispatch treat these methods as part of the target type.
#[derive(Debug)]
pub struct ExtendsDecl {
    /// Type that receives the methods declared in this block.
    pub type_identifier: Ident,
    /// Generic parameters scoped only to this block.
    pub type_parameters: Vec<TypeParameter>,
    pub methods: Vec<(Callable, StmtsBlock)>,
}

#[derive(Debug)]
pub struct ImplementsDecl {
    /// Type that implements the interfaces declared in this block.
    pub type_identifier: Ident,
    /// Generic parameters scoped only to this block.
    pub type_parameters: Vec<TypeParameter>,
    /// Interfaces that this type implements.
    pub interfaces: Vec<Ident>,
    /// Methods that this type implements.
    pub methods: Vec<(Callable, StmtsBlock)>,
}
