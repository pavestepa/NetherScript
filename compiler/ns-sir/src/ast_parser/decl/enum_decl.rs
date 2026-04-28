use ns_ast::EnumDecl;

use crate::ast_parser::Lowerer;
use crate::ast_parser::module::push_impl_for_enum;
use crate::ast_parser::shared::method::lower_enum_method;
use crate::decl::SirDecl;
use crate::diagnostics::SirResult;
use crate::ids::SirEnumId;
use crate::symbols::SirEnumSymbol;

pub(crate) fn lower(lowerer: &mut Lowerer, e: &EnumDecl) -> SirResult<()> {
    let enum_name = e.ident.clone().into_simple().as_str().to_string();
    let enum_id = SirEnumId(lowerer.next_enum_id);
    lowerer.next_enum_id += 1;
    lowerer.enum_by_name.insert(enum_name.clone(), enum_id);
    lowerer.program.enums.push(SirEnumSymbol {
        id: enum_id,
        name: enum_name.clone(),
        members: e
            .members
            .iter()
            .map(|m| m.ident.clone().into_simple().as_str().to_string())
            .collect(),
    });
    lowerer.program.decls.push(SirDecl::Enum { id: enum_id });

    let mut method_ids = Vec::new();
    for method in &e.methods {
        let method_name = method.signature.ident.clone().into_simple().as_str().to_string();
        let fn_id = lower_enum_method(
            lowerer,
            &enum_name,
            &method_name,
            &method.signature.arguments,
            &method.signature.return_type,
            &method.body.stmts,
        );
        method_ids.push(fn_id);
    }
    push_impl_for_enum(lowerer, enum_id, method_ids);
    Ok(())
}
