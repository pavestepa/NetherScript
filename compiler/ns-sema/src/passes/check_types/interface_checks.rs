use ns_ast::{Callable, FunctionDecl, InterfaceDecl, Stmt, This};
use std::collections::HashSet;

use super::checker::{TypeChecker, ident_name};

impl TypeChecker<'_> {
    pub(super) fn check_function_decl(&mut self, d: &FunctionDecl) {
        self.push_scope();
        for tp in &d.signature.type_parameters {
            self.declare_type_param(&ident_name(&tp.ident));
        }
        for arg in &d.signature.arguments {
            let arg_ty = self.type_from_node(&arg.type_ref);
            self.declare_value(&ident_name(&arg.ident), arg_ty);
        }
        let return_ty = self.type_from_node(&d.signature.return_type);
        self.check_block(&d.body.stmts, Some(return_ty.clone()));
        if self.requires_explicit_return(&return_ty) && !self.block_guarantees_return(&d.body.stmts) {
            self.report(
                "E0641",
                format!(
                    "function `{}` must return `{}` on all paths",
                    ident_name(&d.signature.ident),
                    self.type_name(&return_ty)
                ),
                format!("function {}", ident_name(&d.signature.ident)),
                vec!["add `return <expr>` in all control-flow paths".to_string()],
            );
        }
        self.pop_scope();
    }

    pub(super) fn check_interface_decl(&mut self, i: &InterfaceDecl) {
        let interface_name = ident_name(&i.ident);
        let mut method_names = HashSet::new();
        for (sig, maybe_body) in &i.methods {
            let method_name = ident_name(&sig.ident);
            if !method_names.insert(method_name.clone()) {
                self.report(
                    "E0626",
                    format!("duplicate method `{method_name}` in interface `{interface_name}`"),
                    format!("interface {interface_name} method {method_name}"),
                    vec![],
                );
            }

            if let Some(body) = maybe_body {
                self.push_scope();
                for tp in &i.type_parameters {
                    self.declare_type_param(&ident_name(&tp.ident));
                }
                for tp in &sig.type_parameters {
                    self.declare_type_param(&ident_name(&tp.ident));
                }
                for arg in &sig.arguments {
                    let arg_ty = self.type_from_node(&arg.type_ref);
                    self.declare_value(&ident_name(&arg.ident), arg_ty);
                }
                let return_ty = self.type_from_node(&sig.return_type);
                self.check_block(&body.stmts, Some(return_ty.clone()));
                if self.requires_explicit_return(&return_ty) && !self.block_guarantees_return(&body.stmts) {
                    self.report(
                        "E0641",
                        format!(
                            "method `{}` must return `{}` on all paths",
                            ident_name(&sig.ident),
                            self.type_name(&return_ty)
                        ),
                        format!("method {}", ident_name(&sig.ident)),
                        vec!["add `return <expr>` in all control-flow paths".to_string()],
                    );
                }
                self.pop_scope();
            }
        }
    }

    pub(super) fn check_interface_method_compatibility(
        &mut self,
        impl_type_name: &str,
        interface_name: &str,
        method_name: &str,
        required: &Callable,
        actual: &Callable,
    ) {
        let required_receiver = matches!(required.this, This::Receiver(_));
        let actual_receiver = matches!(actual.this, This::Receiver(_));
        if required_receiver != actual_receiver {
            self.report(
                "E0625",
                format!(
                    "method `{method_name}` in type `{impl_type_name}` has incompatible receiver for interface `{interface_name}`"
                ),
                format!("method {method_name}"),
                vec![],
            );
        }

        if required.arguments.len() != actual.arguments.len() {
            self.report(
                "E0622",
                format!(
                    "method `{method_name}` in type `{impl_type_name}` has {} args, interface `{interface_name}` expects {}",
                    actual.arguments.len(),
                    required.arguments.len()
                ),
                format!("method {method_name}"),
                vec![],
            );
            return;
        }

        for idx in 0..required.arguments.len() {
            let expected = self.type_from_node(&required.arguments[idx].type_ref);
            let got = self.type_from_node(&actual.arguments[idx].type_ref);
            if !(self.is_assignable(&expected, &got) && self.is_assignable(&got, &expected)) {
                self.report(
                    "E0623",
                    format!(
                        "method `{method_name}` argument #{idx} in type `{impl_type_name}` does not match interface `{interface_name}`"
                    ),
                    format!("method {method_name} arg #{idx}"),
                    vec![
                        format!("expected `{}`", self.type_name(&expected)),
                        format!("found `{}`", self.type_name(&got)),
                    ],
                );
            }
        }

        let expected_ret = self.type_from_node(&required.return_type);
        let got_ret = self.type_from_node(&actual.return_type);
        if !(self.is_assignable(&expected_ret, &got_ret) && self.is_assignable(&got_ret, &expected_ret)) {
            self.report(
                "E0624",
                format!(
                    "method `{method_name}` in type `{impl_type_name}` has incompatible return type for interface `{interface_name}`"
                ),
                format!("method {method_name} return type"),
                vec![
                    format!("expected `{}`", self.type_name(&expected_ret)),
                    format!("found `{}`", self.type_name(&got_ret)),
                ],
            );
        }
    }
}

impl TypeChecker<'_> {
    fn requires_explicit_return(&self, return_ty: &super::types::CheckedType) -> bool {
        match return_ty {
            super::types::CheckedType::Resolved(id) => {
                *id != self.builtins.void && *id != self.builtins.never
            }
            super::types::CheckedType::Callable { .. } => true,
            super::types::CheckedType::Error => false,
        }
    }

    fn block_guarantees_return(&self, stmts: &[Stmt]) -> bool {
        stmts.iter().any(|stmt| self.stmt_guarantees_return(stmt))
    }

    fn stmt_guarantees_return(&self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::Return(_) => true,
            Stmt::If(s) => {
                let then_returns = self.block_guarantees_return(&s.body.stmts);
                let else_returns = s
                    .alt
                    .as_ref()
                    .map(|alt| self.block_guarantees_return(&alt.stmts))
                    .unwrap_or(false);
                then_returns && else_returns
            }
            _ => false,
        }
    }
}
