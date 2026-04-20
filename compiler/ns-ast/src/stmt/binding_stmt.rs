use crate::{Binding, Expr};

#[derive(Debug)]
pub struct BindingStmt {
    pub is_let: bool,
    pub binding: Binding,
    pub value: Box<Expr>,
}

impl BindingStmt {
    pub fn new(is_let: bool, binding: Binding, value: Box<Expr>) -> Self {
        Self {
            is_let,
            binding,
            value,
        }
    }
}
