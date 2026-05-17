use crate::{Expr, Ident};

/// Constructor invocation, e.g. `new AClass(...)`.
#[derive(Debug, Clone)]
pub struct NewExpr {
    pub class_ident: Ident,
    pub args: Vec<Expr>,
}

impl NewExpr {
    pub fn new(class_ident: Ident, args: Vec<Expr>) -> Self {
        Self { class_ident, args }
    }
}
