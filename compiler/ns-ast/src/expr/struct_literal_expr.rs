use crate::{Expr, Ident};

/// One named field initializer inside a structural literal.
#[derive(Debug, Clone)]
pub struct StructLiteralField {
    pub ident: Ident,
    pub value: Box<Expr>,
}

/// Value built by naming a type and listing field initializers for that type.
#[derive(Debug, Clone)]
pub struct StructLiteralExpr {
    pub struct_name: Ident,
    pub fields: Vec<StructLiteralField>,
}
