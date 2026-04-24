use crate::{Expr, Ident};

#[derive(Debug, Clone)]
pub struct MemberExpr {
    object: Box<Expr>,
    property: MemberProperty,
}

impl MemberExpr {
    pub fn new(object: Expr, property: MemberProperty) -> Self {
        Self {
            object: Box::new(object),
            property,
        }
    }
}

/// How a member is selected on the object expression.
#[derive(Debug, Clone)]
pub enum MemberProperty {
    Ident(Ident),    // obj.name
    Expr(Box<Expr>), // obj[key]
}
