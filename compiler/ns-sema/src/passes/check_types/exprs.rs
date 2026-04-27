use ns_ast::{BinaryOperator, Expr, LiteralExpr, LogicalOperator, MemberProperty};

use super::{checker::TypeChecker, types::CheckedType};

impl TypeChecker<'_> {
    pub(super) fn check_expr(&mut self, expr: &Expr) -> CheckedType {
        match expr {
            Expr::LiteralExpr(lit) => match lit {
                LiteralExpr::Number(_) => CheckedType::Resolved(self.builtins.i32),
                LiteralExpr::String(_) => CheckedType::Resolved(self.builtins.string),
                LiteralExpr::Boolean(_) => CheckedType::Resolved(self.builtins.boolean),
            },
            Expr::BindingExpr(b) => {
                let name = super::checker::ident_name(&b.0);
                let ty = self.lookup_value(&name).unwrap_or_else(|| {
                    // E0604: value is missing from checker value environment.
                    self.report(
                        "E0604",
                        format!("unknown value `{name}` during type checking"),
                        format!("{expr:?}"),
                        vec!["name resolution should have reported this already".to_string()],
                    );
                    CheckedType::Error
                });
                if !matches!(ty, CheckedType::Error)
                    && matches!(self.is_definitely_initialized(&name), Some(false))
                {
                    // E0637: use of local variable before guaranteed initialization.
                    self.report(
                        "E0637",
                        format!("use of possibly uninitialized variable `{name}`"),
                        format!("{expr:?}"),
                        vec![
                            "variable is declared without initializer".to_string(),
                            "initialize it on all control-flow paths before first use".to_string(),
                        ],
                    );
                    return CheckedType::Error;
                }
                ty
            }
            Expr::UnaryExpr(u) => match u {
                ns_ast::UnaryExpr::Minus(v) | ns_ast::UnaryExpr::Plus(v) => {
                    let t = self.check_expr(v);
                    if !self.is_builtin_type(&t, self.builtins.i32) && !matches!(t, CheckedType::Error) {
                        // E0605: numeric unary operator used on non-i32.
                        self.report(
                            "E0605",
                            "numeric unary operator expects i32".to_string(),
                            format!("{expr:?}"),
                            vec![format!("found `{}`", self.type_name(&t))],
                        );
                        CheckedType::Error
                    } else {
                        CheckedType::Resolved(self.builtins.i32)
                    }
                }
                ns_ast::UnaryExpr::Not(v) | ns_ast::UnaryExpr::BitNot(v) => {
                    let t = self.check_expr(v);
                    if !self.is_builtin_type(&t, self.builtins.boolean) && !matches!(t, CheckedType::Error) {
                        // E0606: logical unary operator used on non-boolean.
                        self.report(
                            "E0606",
                            "logical unary operator expects boolean".to_string(),
                            format!("{expr:?}"),
                            vec![format!("found `{}`", self.type_name(&t))],
                        );
                        CheckedType::Error
                    } else {
                        CheckedType::Resolved(self.builtins.boolean)
                    }
                }
                ns_ast::UnaryExpr::Deref(v) => self.check_expr(v),
            },
            Expr::BinaryExpr(b) => {
                let left = self.check_expr(&b.left);
                let right = self.check_expr(&b.right);
                match b.kind {
                    BinaryOperator::Plus
                    | BinaryOperator::Minus
                    | BinaryOperator::Star
                    | BinaryOperator::Slash
                    | BinaryOperator::Percent => {
                        if (!self.is_builtin_type(&left, self.builtins.i32)
                            && !matches!(left, CheckedType::Error))
                            || (!self.is_builtin_type(&right, self.builtins.i32)
                                && !matches!(right, CheckedType::Error))
                        {
                            // E0607: arithmetic operators require i32 operands.
                            self.report(
                                "E0607",
                                "arithmetic operator expects i32 operands".to_string(),
                                format!("{expr:?}"),
                                vec![
                                    format!("left is `{}`", self.type_name(&left)),
                                    format!("right is `{}`", self.type_name(&right)),
                                ],
                            );
                            CheckedType::Error
                        } else {
                            CheckedType::Resolved(self.builtins.i32)
                        }
                    }
                }
            }
            Expr::LogicalExpr(l) => {
                let left = self.check_expr(&l.left);
                let right = self.check_expr(&l.right);
                match l.kind {
                    LogicalOperator::Equals
                    | LogicalOperator::NotEquals
                    | LogicalOperator::Less
                    | LogicalOperator::Greater
                    | LogicalOperator::LessEqual
                    | LogicalOperator::GreaterEqual => {
                        if !self.is_assignable(&left, &right) && !self.is_assignable(&right, &left) {
                            // E0608: comparison operands are type-incompatible.
                            self.report(
                                "E0608",
                                "logical comparison expects compatible operands".to_string(),
                                format!("{expr:?}"),
                                vec![
                                    format!("left is `{}`", self.type_name(&left)),
                                    format!("right is `{}`", self.type_name(&right)),
                                ],
                            );
                        }
                        CheckedType::Resolved(self.builtins.boolean)
                    }
                }
            }
            Expr::CallExpr(c) => {
                let callee_ty = self.check_expr(&c.callee);

                if let Expr::BindingExpr(be) = c.callee.as_ref() {
                    let func_name = super::checker::ident_name(&be.0);
                    if func_name == "print" {
                        for arg in &c.args {
                            let _ = self.check_expr(arg);
                        }
                        return CheckedType::Resolved(self.builtins.void);
                    }
                    if func_name == "panic" {
                        for arg in &c.args {
                            let _ = self.check_expr(arg);
                        }
                        return CheckedType::Resolved(self.builtins.never);
                    }
                }

                if let CheckedType::Callable { params, ret } = callee_ty {
                    if params.len() != c.args.len() {
                        self.report(
                            "E0609",
                            format!(
                                "callable expects {} args, got {}",
                                params.len(),
                                c.args.len()
                            ),
                            format!("{expr:?}"),
                            vec![],
                        );
                    }
                    let pair_count = params.len().min(c.args.len());
                    for idx in 0..pair_count {
                        let expected = CheckedType::Resolved(params[idx]);
                        let got = self.check_expr(&c.args[idx]);
                        self.expect_assignable(
                            &expected,
                            &got,
                            "call argument type mismatch",
                            format!("arg #{idx} in callable call"),
                        );
                    }
                    CheckedType::Resolved(ret)
                } else {
                    self.report(
                        "E0610",
                        "callee expression is not callable".to_string(),
                        format!("{expr:?}"),
                        vec!["expected function or method value".to_string()],
                    );
                    CheckedType::Error
                }
            }
            Expr::NewExpr(n) => {
                let cls = super::checker::ident_name(&n.class_ident);
                if !self.class_names.contains(&cls) {
                    // E0617: `new` must target a known class declaration.
                    self.report(
                        "E0617",
                        format!("cannot instantiate unknown class `{cls}`"),
                        format!("{expr:?}"),
                        vec!["`new` currently supports class declarations only".to_string()],
                    );
                }
                for arg in &n.args {
                    let _ = self.check_expr(arg);
                }
                match self.global_types.get(&cls) {
                    Some(id) => CheckedType::Resolved(*id),
                    None => CheckedType::Error,
                }
            }
            Expr::StructLiteral(s) => {
                let name = super::checker::ident_name(&s.struct_name);
                for field in &s.fields {
                    let _ = self.check_expr(&field.value);
                }
                match self.global_types.get(&name) {
                    Some(id) => CheckedType::Resolved(*id),
                    None => CheckedType::Error,
                }
            }
            Expr::Referencing(r) => self.check_expr(&r.expr),
            Expr::MemberExpr(m) => {
                let object_ty = self.check_expr(m.object());
                match m.property() {
                    MemberProperty::Expr(index) => {
                        let _ = self.check_expr(index);
                        self.report(
                            "E0636",
                            "computed member access typing is not implemented".to_string(),
                            format!("{m:?}"),
                            vec![],
                        );
                        CheckedType::Error
                    }
                    MemberProperty::Ident(property) => {
                        let Some(obj_name) = self.checked_object_name(&object_ty) else {
                            self.report(
                                "E0635",
                                "member access on non-object type".to_string(),
                                format!("{m:?}"),
                                vec![format!("object type is `{}`", self.type_name(&object_ty))],
                            );
                            return CheckedType::Error;
                        };
                        let prop_name = super::checker::ident_name(property);

                        if let Some(methods) = self.type_methods.get(&obj_name) {
                            if let Some(sig) = methods.get(&prop_name).cloned() {
                                return self.callable_type_from_signature(&sig);
                            }
                        }

                        if let Some(fields) = self.type_fields.get(&obj_name) {
                            if let Some(field_ty) = fields.get(&prop_name) {
                                return field_ty
                                    .map(CheckedType::Resolved)
                                    .unwrap_or(CheckedType::Error);
                            }
                        }

                        self.report(
                            "E0635",
                            format!("unknown member `{prop_name}` for type `{obj_name}`"),
                            format!("{m:?}"),
                            vec![],
                        );
                        CheckedType::Error
                    }
                }
            }
        }
    }

    fn is_builtin_type(&self, ty: &CheckedType, builtin: crate::types::TypeId) -> bool {
        matches!(ty, CheckedType::Resolved(id) if *id == builtin)
    }

    fn checked_object_name(&self, ty: &CheckedType) -> Option<String> {
        match ty {
            CheckedType::Resolved(id) => self.type_leaf_name(*id),
            _ => None,
        }
    }
}
