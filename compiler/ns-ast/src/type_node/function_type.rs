use crate::type_node::{TypeNode, TypeParameter};

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<TypeNode>,
    pub return_type: Box<TypeNode>,
}

// " (i32, String) => String " like this
