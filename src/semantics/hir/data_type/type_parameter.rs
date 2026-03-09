use crate::{semantics::hir::data_type::DataType, Atom};

pub struct TypeParameter {
    id: Atom,
    constaint: Option<DataType>,
    default_type: Option<DataType>,
}
