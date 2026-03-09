use crate::{semantics::hir::data_type::DataType, Atom};

pub struct EnumMember {
    pub id: Atom,
    pub contains: Option<DataType>,
}
