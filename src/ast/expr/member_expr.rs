use crate::{ast::expr::CallExpr, Atom};

use super::Expr;

#[derive(Debug)]
pub struct MemberExpr {
    object: Box<Expr>,
    property: Atom,
}
