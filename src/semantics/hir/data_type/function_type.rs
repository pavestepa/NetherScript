use crate::semantics::hir::data_type::{type_parameter::TypeParameter, DataType};

pub struct FunctionType {
    type_parameters: Vec<TypeParameter>,
    parameters: Vec<DataType>,
    return_type: Box<DataType>,
}

// " (i32, String) => String " like this
