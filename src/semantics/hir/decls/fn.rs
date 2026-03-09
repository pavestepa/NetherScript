use crate::{
    semantics::hir::{binding::Binding, data_type::DataType, stmts::Stmts},
    Atom,
};

pub struct FnDecl {
    id: Atom,
    args: Vec<Binding>,
    return_type: DataType,
    body: Stmts,
}
