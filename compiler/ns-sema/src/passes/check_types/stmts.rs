use ns_ast::{AssignTarget, Stmt};

use super::{checker::TypeChecker, types::CheckedType};

impl TypeChecker<'_> {
    pub(super) fn check_stmt(&mut self, stmt: &Stmt, return_ty: Option<CheckedType>) {
        match stmt {
            Stmt::Error(_) => {}
            Stmt::Binding(s) => {
                let binding_ty = s.binding.type_ref.as_ref().map(|t| self.type_from_node(t));

                if let Some(value) = &s.value {
                    let value_ty = self.check_expr(value);
                    let declared_or_inferred_ty =
                        binding_ty.clone().unwrap_or_else(|| value_ty.clone());
                    self.expect_assignable(
                        &declared_or_inferred_ty,
                        &value_ty,
                        "binding type mismatch",
                        format!("binding {}", super::checker::ident_name(&s.binding.ident)),
                    );
                    self.declare_value(
                        &super::checker::ident_name(&s.binding.ident),
                        declared_or_inferred_ty,
                    );
                    self.declare_value_init(&super::checker::ident_name(&s.binding.ident), true);
                } else if s.is_let {
                    let declared_ty = binding_ty.unwrap_or(CheckedType::Error);
                    if matches!(declared_ty, CheckedType::Error) {
                        self.report(
                            "E0604",
                            "let without initializer must have an explicit type".to_string(),
                            format!("binding {}", super::checker::ident_name(&s.binding.ident)),
                            vec![
                                "add a type annotation or provide an initializer".to_string(),
                            ],
                        );
                    }
                    self.declare_value(&super::checker::ident_name(&s.binding.ident), declared_ty);
                    self.declare_value_init(&super::checker::ident_name(&s.binding.ident), false);
                } else {
                    self.report(
                        "E0605",
                        "const binding requires an initializer".to_string(),
                        format!("binding {}", super::checker::ident_name(&s.binding.ident)),
                        vec!["provide `= <expr>`".to_string()],
                    );
                    self.declare_value(
                        &super::checker::ident_name(&s.binding.ident),
                        binding_ty.unwrap_or(CheckedType::Error),
                    );
                    self.declare_value_init(&super::checker::ident_name(&s.binding.ident), false);
                }
            }
            Stmt::Assign(s) => {
                let rhs = self.check_expr(&s.assign);
                match &s.target {
                    AssignTarget::Ident(id) => {
                        let name = super::checker::ident_name(id);
                        let lhs = self
                            .lookup_value(&name)
                            .unwrap_or(CheckedType::Error);
                        self.expect_assignable(&lhs, &rhs, "assignment type mismatch", format!("{id:?}"));
                        let _ = self.mark_initialized(&name);
                    }
                    AssignTarget::Member(member) => {
                        // E0601: member assignment typing is blocked until MemberExpr API exposes object/property.
                        self.report(
                            "E0601",
                            "member assignment type check is not available yet".to_string(),
                            format!("{member:?}"),
                            vec![
                                "Expose `MemberExpr` internals in ns-ast to enable precise member typing"
                                    .to_string(),
                            ],
                        );
                    }
                }
            }
            Stmt::Expr(s) => {
                let _ = self.check_expr(&s.expr);
            }
            Stmt::If(s) => {
                let cond_ty = self.check_expr(&s.test);
                if !matches!(cond_ty, CheckedType::Error)
                    && !matches!(cond_ty, CheckedType::Resolved(id) if id == self.builtins.boolean)
                {
                    // E0602: if condition must be boolean.
                    self.report(
                        "E0602",
                        "if condition must be boolean".to_string(),
                        format!("{:?}", s.test),
                        vec![format!("found `{}`", self.type_name(&cond_ty))],
                    );
                }
                let state_before_if = self.init_scopes.clone();

                self.check_block(&s.body.stmts, return_ty.clone());
                let then_state = self.init_scopes.clone();

                self.init_scopes = state_before_if.clone();
                if let Some(alt) = &s.alt {
                    self.check_block(&alt.stmts, return_ty.clone());
                }
                let else_state = self.init_scopes.clone();

                self.init_scopes = merge_init_states(&then_state, &else_state, &state_before_if);
            }
            Stmt::Loop(s) => {
                let state_before_loop = self.init_scopes.clone();
                self.check_block(&s.body.stmts, return_ty);
                self.init_scopes = state_before_loop;
            }
            Stmt::While(s) => {
                let cond_ty = self.check_expr(&s.test);
                if !matches!(cond_ty, CheckedType::Error)
                    && !matches!(cond_ty, CheckedType::Resolved(id) if id == self.builtins.boolean)
                {
                    // E0603: while condition must be boolean.
                    self.report(
                        "E0603",
                        "while condition must be boolean".to_string(),
                        format!("{:?}", s.test),
                        vec![format!("found `{}`", self.type_name(&cond_ty))],
                    );
                }
                let state_before_while = self.init_scopes.clone();
                self.check_block(&s.body.stmts, return_ty);
                self.init_scopes = state_before_while;
            }
            Stmt::Return(s) => {
                let expected = return_ty.unwrap_or(CheckedType::Resolved(self.builtins.void));
                let got = s
                    .arg
                    .as_ref()
                    .map(|e| self.check_expr(e))
                    .unwrap_or(CheckedType::Resolved(self.builtins.void));
                self.expect_assignable(&expected, &got, "return type mismatch", "return".to_string());
            }
            Stmt::Break(_) => {}
        }
    }
}

fn merge_init_states(
    then_state: &[std::collections::HashMap<String, bool>],
    else_state: &[std::collections::HashMap<String, bool>],
    base_state: &[std::collections::HashMap<String, bool>],
) -> Vec<std::collections::HashMap<String, bool>> {
    let scope_count = base_state.len().min(then_state.len()).min(else_state.len());
    let mut merged = Vec::with_capacity(scope_count);

    for scope_idx in 0..scope_count {
        let mut merged_scope = std::collections::HashMap::new();
        for name in base_state[scope_idx].keys() {
            let then_init = then_state[scope_idx].get(name).copied().unwrap_or(false);
            let else_init = else_state[scope_idx].get(name).copied().unwrap_or(false);
            merged_scope.insert(name.clone(), then_init && else_init);
        }
        merged.push(merged_scope);
    }

    if merged.is_empty() {
        merged.push(std::collections::HashMap::new());
    }

    merged
}
