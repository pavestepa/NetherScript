use ns_ast::ConstDecl;

use crate::ast_parser::Lowerer;
use crate::decl::SirDecl;
use crate::diagnostics::SirResult;

pub(crate) fn lower(lowerer: &mut Lowerer, c: &ConstDecl) -> SirResult<()> {
    let name = c.binding.ident.clone().into_simple().as_str().to_string();
    let type_id = lowerer.lower_type(&c.binding.type_ref);
    let sid = lowerer
        .lookup_value(&name)
        .unwrap_or_else(|| lowerer.alloc_value(&name, Some(type_id)));
    if let Some(symbol) = lowerer.program.values.iter_mut().find(|value| value.id == sid) {
        symbol.ty = Some(type_id);
    }
    lowerer.declare_value(name, sid);
    let expr = lowerer.lower_expr(&c.val);
    lowerer.program.decls.push(SirDecl::Const { value: sid, expr });
    Ok(())
}
