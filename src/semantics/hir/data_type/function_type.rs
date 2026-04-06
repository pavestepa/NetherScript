use crate::semantics::hir::data_type::{type_parameter::TypeParameter, DataType};

pub struct FnType {
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<DataType>,
    pub return_type: Box<DataType>,
}

// " (i32, String) => String " like this
