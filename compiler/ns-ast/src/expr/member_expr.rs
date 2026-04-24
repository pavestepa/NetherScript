use crate::{Expr, Ident};

#[derive(Debug, Clone)]
pub struct MemberExpr {
    object: Box<Expr>,
    property: MemberProperty,
}

#[derive(Debug, Clone)]
pub enum MemberProperty {
    Ident(Ident),      // obj.name
    Expr(Box<Expr>),   // obj[key]
}
