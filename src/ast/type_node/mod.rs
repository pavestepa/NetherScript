mod function_type;
mod generic_type;
mod tuple_type;
mod type_parameter;

pub use {
    function_type::FunctionType, generic_type::GenericType, tuple_type::TupleType,
    type_parameter::TypeParameter,
};

#[derive(Debug, Clone)]
pub enum TypeNode {
    GenericType(GenericType),
    FunctionType(FunctionType),
    TupleType(TupleType),
}
