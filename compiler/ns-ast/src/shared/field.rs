use crate::{Binding, Expr};

#[derive(Debug)]
pub struct Field {
    binding: Binding,
    init: Option<Expr>,
}
