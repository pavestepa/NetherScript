use crate::ast::Expr;

#[derive(Debug, Clone)]
pub enum AssignOp {
    Assign(Box<Expr>),
}
