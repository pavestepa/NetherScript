use ns_ast::NamedType;

use crate::ast_parser::Lowerer;
use crate::types::SirType;

pub(crate) fn lower(lowerer: &mut Lowerer, n: &NamedType) -> SirType {
    let name = n.ident.clone().into_simple().as_str().to_string();
    if n.type_arguments.is_empty() && lowerer.is_type_param(&name) {
        return SirType::TypeParam { name };
    }
    SirType::Named {
        name,
        args: n.type_arguments.iter().map(|arg| lowerer.lower_type(arg)).collect(),
    }
}
