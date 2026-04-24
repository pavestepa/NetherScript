use crate::{Expr, Ident};

/// One field in a struct literal: `id: expr` (Rust-style; shorthand `id` can be lowered to `id: id`).
#[derive(Debug, Clone)]
pub struct StructLiteralField {
    pub ident: Ident,
    pub value: Box<Expr>,
}

/// Struct / nominal object literal: `User { id: id, name: "Paul" }`.
#[derive(Debug, Clone)]
pub struct StructLiteralExpr {
    pub struct_name: Ident,
    pub fields: Vec<StructLiteralField>,
}
