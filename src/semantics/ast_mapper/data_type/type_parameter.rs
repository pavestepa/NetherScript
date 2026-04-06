use crate::{semantics::hir, syntax::ast};

pub fn type_parameter_mapper(from: ast::TypeParameter) -> hir::data_type::TypeParameter {
    hir::data_type::TypeParameter {
        id: from.ident.0,
        constaint,
    }
}
