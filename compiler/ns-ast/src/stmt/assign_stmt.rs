use crate::{ast::Ast, Expr, Ident};

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub ident: Ident,
    pub assign: Box<Ast<Expr>>,
}
