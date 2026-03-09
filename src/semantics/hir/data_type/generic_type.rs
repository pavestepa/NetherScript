use crate::{semantics::hir::data_type::DataType, Atom};

pub struct GenericType {
    pub id: Atom,
    pub arguments: Option<Vec<DataType>>,
}
