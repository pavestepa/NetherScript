use crate::{semantics::hir, syntax::ast};

pub fn fn_type_mapper(from: ast::FunctionType) -> hir::data_type::FnType {
    let mut prms = vec![];
    for prm in from.type_parameters {
        prms.push(type_parameter_mapper(prm));
    }
    hir::data_type::FnType {
        type_parameters: prms,
    }
}
