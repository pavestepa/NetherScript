use ns_ast::FunctionDecl;

use crate::ast_parser::Lowerer;
use crate::decl::SirDecl;
use crate::diagnostics::SirResult;
use crate::symbols::SirFunctionSymbol;

pub(crate) fn lower(lowerer: &mut Lowerer, f: &FunctionDecl) -> SirResult<()> {
    let fn_name = f.signature.ident.clone().into_simple().as_str().to_string();
    let fn_id = lowerer
        .find_function_id(&fn_name)
        .unwrap_or_else(|| lowerer.alloc_fn_symbol(fn_name.clone(), Vec::new(), None));

    lowerer.push_scope();
    lowerer.push_type_param_scope();
    for type_param in &f.signature.type_parameters {
        let name = type_param.ident.clone().into_simple().as_str().to_string();
        lowerer.declare_type_param(&name);
    }
    let mut params = Vec::with_capacity(f.signature.arguments.len());
    for arg in &f.signature.arguments {
        let arg_name = arg.ident.clone().into_simple().as_str().to_string();
        let arg_ty = lowerer.lower_type(&arg.type_ref);
        let arg_id = lowerer.alloc_value(&arg_name, Some(arg_ty));
        lowerer.declare_value(arg_name, arg_id);
        params.push(arg_id);
    }
    let ret = Some(lowerer.lower_type(&f.signature.return_type));
    let body = f.body.stmts.iter().map(|s| lowerer.lower_stmt(s)).collect();
    lowerer.pop_type_param_scope();
    lowerer.pop_scope();

    if let Some(symbol) = lowerer.function_symbol_mut(fn_id) {
        symbol.name = fn_name;
        symbol.params = params;
        symbol.ret = ret;
    } else {
        lowerer.program.functions.push(SirFunctionSymbol {
            id: fn_id,
            name: fn_name,
            params,
            ret,
        });
    }
    lowerer.program.decls.push(SirDecl::Function { id: fn_id, body });
    Ok(())
}
