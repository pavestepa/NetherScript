use crate::ast::Expr;

#[derive(Debug)]
pub struct ReturnStmt {
    pub arg: Option<Box<Expr>>,
}
