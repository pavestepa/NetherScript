mod fn_type;
mod generic_type;
mod reference_type;
mod tuple_type;
mod type_parameter;

use crate::{semantics::hir, syntax::ast};

pub fn data_type_mapper(from: ast::TypeNode) -> hir::data_type::DataType {
    match from {
        ast::TypeNode::FunctionType(val) => hir::data_type::DataType::FnType(),
        ast::TypeNode::GenericType(val) => hir::data_type::DataType::GenericType(()),
        ast::TypeNode::ReferenceType(val) => hir::data_type::DataType::ReferenceType(()),
        ast::TypeNode::TupleType(val) => hir::data_type::DataType::TupleType(()),
    }
}
