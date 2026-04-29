use crate::{Expr, RefKind};

/// Expression that takes the address of another expression according to the given reference mode.
#[derive(Debug, Clone)]
pub struct Referencing {
    pub ref_kind: RefKind,
    pub expr: Box<Expr>,
}

impl Referencing {
    pub fn new(ref_kind: RefKind, expr: Expr) -> Self {
        Self {
            ref_kind: ref_kind,
            expr: Box::new(expr),
        }
    }
}
