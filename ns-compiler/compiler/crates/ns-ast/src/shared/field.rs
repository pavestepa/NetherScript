use crate::{Binding, Expr};

/// Class field in TypeScript style: optional type annotation and optional initializer.
#[derive(Debug, Clone)]
pub struct Field {
    pub binding: Binding,
    pub init: Option<Expr>,
}
