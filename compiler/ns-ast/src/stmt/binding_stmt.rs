use crate::{Binding, Expr};

#[derive(Debug, Clone)]
pub struct BindingStmt {
    pub is_let: bool,
    pub binding: Binding,
    pub value: Option<Box<Expr>>,
}

impl BindingStmt {
    pub fn new(is_let: bool, binding: Binding, value: Option<Box<Expr>>) -> Self {
        Self {
            is_let,
            binding,
            value,
        }
    }
}
