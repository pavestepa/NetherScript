use crate::ast::{ast::Ast, Expr, RefKind};

#[derive(Debug, Clone)]
pub struct Referencing {
    pub ref_kind: RefKind,
    pub expr: Box<Ast<Expr>>,
}
