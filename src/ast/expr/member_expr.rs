use crate::{ast::expr::CallExpr, Atom};

use super::Expr;

#[derive(Debug)]
pub struct MemberExpr {
    object: Box<Expr>,
    prop: Box<Expr>, // .x | .x()
}

impl MemberExpr {
    pub fn new(object: Expr, prop: Expr) -> Self {
        Self {
            object: Box::new(object),
            prop: Box::new(prop),
        }
    }
}
