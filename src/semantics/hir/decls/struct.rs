use crate::{semantics::hir::binding::Binding, Atom};

pub struct StructDecl {
    id: Atom,
    body: Vec<Binding>,
}
