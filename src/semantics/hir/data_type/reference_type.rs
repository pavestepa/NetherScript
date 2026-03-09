use crate::semantics::hir::{data_type::DataType, ref_kind::RefKind};

pub struct ReferenceType {
    pub kind: RefKind,
    pub argument: Box<DataType>,
}
