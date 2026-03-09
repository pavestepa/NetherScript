use crate::{semantics::hir::enum_member::EnumMember, Atom};

pub struct EnumDecl {
    id: Atom,
    body: Vec<EnumMember>,
}
