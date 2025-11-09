use crate::{Atom, ast::operators::BinaryOperator};

#[derive(Debug)]
pub enum Expr {
    NumberLiteral(Atom),
    StringLiteral(Atom),
    Identifier(Atom),
    ObjectLiteral(Vec<Property>),
    Binary {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Member {
        object: Box<Expr>,
        properties: Option<Vec<Expr>>,
    },
}

#[derive(Debug)]
pub struct Property {
    pub key: Atom,
    pub value: Expr,
}