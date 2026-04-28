use ns_ast::{Stmt, TypeNode, TypedBinding};

use crate::ast_parser::Lowerer;
use crate::decl::SirDecl;
use crate::ids::{SirClassId, SirFnId};

pub(crate) fn lower_class_method(
    lowerer: &mut Lowerer,
    class_id: SirClassId,
    method_name: &str,
    arguments: &[TypedBinding],
    return_type: &TypeNode,
    body_stmts: &[Stmt],
) -> SirFnId {
    lowerer.push_scope();
    let this_id = lowerer.alloc_value("this", None);
    lowerer.declare_value("this".to_string(), this_id);

    let mut params = Vec::with_capacity(arguments.len() + 1);
    params.push(this_id);
    for arg in arguments {
        let arg_name = arg.ident.clone().into_simple().as_str().to_string();
        let arg_ty = lowerer.lower_type(&arg.type_ref);
        let arg_id = lowerer.alloc_value(&arg_name, Some(arg_ty));
        lowerer.declare_value(arg_name, arg_id);
        params.push(arg_id);
    }
    let ret = Some(lowerer.lower_type(return_type));
    let body = body_stmts.iter().map(|s| lowerer.lower_stmt(s)).collect::<Vec<_>>();
    lowerer.pop_scope();

    let full_name = format!("{}::{}", lowerer.class_name(class_id).unwrap_or("<class>"), method_name);
    let fn_id = lowerer.alloc_fn_symbol(full_name, params, ret);
    lowerer.program.decls.push(SirDecl::Function { id: fn_id, body });
    fn_id
}

pub(crate) fn lower_enum_method(
    lowerer: &mut Lowerer,
    enum_name: &str,
    method_name: &str,
    arguments: &[TypedBinding],
    return_type: &TypeNode,
    body_stmts: &[Stmt],
) -> SirFnId {
    lowerer.push_scope();
    let mut params = Vec::with_capacity(arguments.len());
    for arg in arguments {
        let arg_name = arg.ident.clone().into_simple().as_str().to_string();
        let arg_ty = lowerer.lower_type(&arg.type_ref);
        let arg_id = lowerer.alloc_value(&arg_name, Some(arg_ty));
        lowerer.declare_value(arg_name, arg_id);
        params.push(arg_id);
    }
    let ret = Some(lowerer.lower_type(return_type));
    let body = body_stmts.iter().map(|s| lowerer.lower_stmt(s)).collect::<Vec<_>>();
    lowerer.pop_scope();

    let fn_id = lowerer.alloc_fn_symbol(format!("{enum_name}::{method_name}"), params, ret);
    lowerer.program.decls.push(SirDecl::Function { id: fn_id, body });
    fn_id
}
