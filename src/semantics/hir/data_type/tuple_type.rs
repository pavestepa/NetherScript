use crate::semantics::hir::data_type::DataType;

pub struct TupleType {
    pub body: Vec<DataType>,
}
