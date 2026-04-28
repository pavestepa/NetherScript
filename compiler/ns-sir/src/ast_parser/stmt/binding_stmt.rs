use ns_ast::BindingStmt;

use crate::ast_parser::Lowerer;
use crate::stmt::SirStmt;

pub(crate) fn lower(lowerer: &mut Lowerer, b: &BindingStmt) -> SirStmt {
    let name = b.binding.ident.clone().into_simple().as_str().to_string();
    let init = b.value.as_ref().map(|e| lowerer.lower_expr(e));
    let ty = b.binding.type_ref.as_ref().map(|t| lowerer.lower_type(t));
    let id = lowerer.alloc_value(&name, ty);
    lowerer.declare_value(name, id);
    SirStmt::Let {
        value: id,
        is_mut: b.is_let,
        init,
    }
}
