use crate::ast::{Expr, Ident};

#[derive(Debug, Clone)]
pub struct AssignStmt {
    ident: Ident,
    assign: Box<Expr>,
}
