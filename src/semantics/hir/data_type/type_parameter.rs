use crate::{semantics::hir::data_type::DataType, Atom};

pub struct TypeParameter {
    pub id: Atom,
    pub constaint: Option<DataType>,
    pub default_type: Option<DataType>,
}
