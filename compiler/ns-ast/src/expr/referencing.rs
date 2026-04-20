use crate::{Expr, RefKind};

#[derive(Debug, Clone)]
pub struct Referencing {
    pub ref_kind: RefKind,
    pub expr: Box<Expr>,
}
