use crate::{semantics::hir::data_type::DataType, Atom};

pub struct Binding {
    pub id: Atom,
    pub type_ref: Option<DataType>,
}
