use ns_ast::TypeNode;

use crate::ast_parser::Lowerer;
use crate::ids::SirTypeId;

mod dynamic_type;
mod named_type;

pub(crate) fn lower_type(lowerer: &mut Lowerer, node: &TypeNode) -> SirTypeId {
    let ty = match node {
        TypeNode::Named(n) => named_type::lower(lowerer, n),
        TypeNode::Dynamic(d) => dynamic_type::lower(d),
    };
    lowerer.intern_type(ty)
}
