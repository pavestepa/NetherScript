use crate::ast::{Expr, RefKind};

#[derive(Debug, Clone)]
pub struct Referencing {
    ref_kind: RefKind,
    expr: Box<Expr>,
}
