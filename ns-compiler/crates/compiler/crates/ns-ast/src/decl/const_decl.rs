use crate::{Expr, TypedBinding};

#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub binding: TypedBinding,
    pub val: Expr,
}

impl ConstDecl {
    pub fn new(binding: TypedBinding, val: Expr) -> Self {
        Self { binding, val }
    }
}
