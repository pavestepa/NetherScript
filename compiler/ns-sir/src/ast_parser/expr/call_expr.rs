use ns_ast::{CallExpr, Expr, MemberProperty};

use crate::ast_parser::Lowerer;
use crate::expr::SirExpr;

pub(crate) fn lower(lowerer: &mut Lowerer, c: &CallExpr) -> SirExpr {
    let args = c.args.iter().map(|e| lowerer.lower_expr(e)).collect::<Vec<_>>();

    if is_console_log_call(c.callee.as_ref()) {
        return SirExpr::IntrinsicPrintln { args, ty: None };
    }

    let type_args = c
        .type_arguments
        .iter()
        .map(|t| lowerer.lower_type(t))
        .collect::<Vec<_>>();
    let callee = match c.callee.as_ref() {
        Expr::BindingExpr(be) => lowerer.find_function_id(be.0.clone().into_simple().as_str().as_ref()),
        _ => None,
    };
    let ret_ty = callee.and_then(|id| lowerer.function_ret_type(id));
    SirExpr::Call {
        callee,
        type_args,
        args,
        ty: ret_ty,
    }
}

fn is_console_log_call(callee: &Expr) -> bool {
    let Expr::MemberExpr(member) = callee else {
        return false;
    };

    let MemberProperty::Ident(property) = member.property() else {
        return false;
    };
    if property.clone().into_simple().as_str().as_ref() != "log" {
        return false;
    }

    let Expr::BindingExpr(object) = member.object() else {
        return false;
    };
    object.0.clone().into_simple().as_str().as_ref() == "console"
}
