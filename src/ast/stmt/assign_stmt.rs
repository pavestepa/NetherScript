use crate::ast::{Expr, Ident};

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub ident: Ident,
    pub assign: Box<Expr>,
}
