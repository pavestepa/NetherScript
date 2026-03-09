use crate::semantics::hir::binding::Binding;

pub struct BindingStmt {
    pub is_var: bool,
    pub val: Binding,
}
