use crate::semantics::hir::data_type::{
    function_type::FunctionType, generic_type::GenericType, reference_type::ReferenceType,
    tuple_type::TupleType,
};

mod function_type;
mod generic_type;
mod reference_type;
mod tuple_type;
mod type_parameter;
pub enum DataType {
    ReferenceType(ReferenceType),
    GenericType(GenericType),
    FunctionType(FunctionType),
    TupleType(TupleType),
}
