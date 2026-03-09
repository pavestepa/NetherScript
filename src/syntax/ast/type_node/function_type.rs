use crate::syntax::ast::{
    ast::Ast,
    type_node::{TypeNode, TypeParameter},
};

#[derive(Debug, Clone)]
pub struct FunctionType {
    type_parameters: Vec<Ast<TypeParameter>>,
    parameters: Vec<TypeNode>,
    return_type: Box<TypeNode>,
}

// " (i32, String) => String " like this
