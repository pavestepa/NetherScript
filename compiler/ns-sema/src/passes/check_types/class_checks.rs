use ns_ast::{ClassDecl, ConstDecl};
use std::collections::HashSet;

use super::checker::{TypeChecker, ident_name};

impl TypeChecker<'_> {
    pub(super) fn check_const_decl(&mut self, d: &ConstDecl) {
        let expected = self.type_from_node(&d.binding.type_ref);
        let got = self.check_expr(&d.val);
        self.expect_assignable(
            &expected,
            &got,
            "const initializer type mismatch",
            format!("const {}", ident_name(&d.binding.ident)),
        );
    }

    pub(super) fn check_class_decl(&mut self, c: &ClassDecl) {
        let class_name = ident_name(&c.ident);
        if let Some(interfaces) = &c.implements {
            for interface in interfaces {
                let interface_name = ident_name(interface);
                if !self.interfaces.contains_key(&interface_name) {
                    self.report(
                        "E0615",
                        format!("class `{class_name}` implements unknown interface `{interface_name}`"),
                        format!("class {class_name} implements {interface_name}"),
                        vec!["implemented targets must be interfaces".to_string()],
                    );
                    continue;
                }

                if let Some(required_methods) = self.interface_methods.get(&interface_name).cloned() {
                    for (required_name, required_sig) in required_methods {
                        let class_method = c
                            .methods
                            .iter()
                            .find(|m| ident_name(&m.signature.ident) == required_name);
                        let Some(class_method) = class_method else {
                            self.report(
                                "E0621",
                                format!(
                                    "class `{class_name}` is missing method `{required_name}` required by interface `{interface_name}`"
                                ),
                                format!("class {class_name} implements {interface_name}"),
                                vec![],
                            );
                            continue;
                        };
                        self.check_interface_method_compatibility(
                            &class_name,
                            &interface_name,
                            &required_name,
                            &required_sig,
                            &class_method.signature,
                        );
                    }
                }
            }
        }

        let mut field_names = HashSet::new();
        for f in &c.fields {
            let field_name = ident_name(&f.binding.ident);
            if !field_names.insert(field_name.clone()) {
                self.report(
                    "E0618",
                    format!("duplicate field `{field_name}` in class `{class_name}`"),
                    format!("field {field_name}"),
                    vec![],
                );
            }

            if let Some(init) = &f.init {
                let expected = f
                    .binding
                    .type_ref
                    .as_ref()
                    .map(|t| self.type_from_node(t))
                    .unwrap_or_else(|| self.check_expr(init));
                let got = self.check_expr(init);
                self.expect_assignable(
                    &expected,
                    &got,
                    "class field initializer type mismatch",
                    format!("field {}", ident_name(&f.binding.ident)),
                );
            }
        }

        let mut method_names = HashSet::new();
        for m in &c.methods {
            let method_name = ident_name(&m.signature.ident);
            if !method_names.insert(method_name.clone()) {
                self.report(
                    "E0619",
                    format!("duplicate method `{method_name}` in class `{class_name}`"),
                    format!("method {method_name}"),
                    vec![],
                );
            }

            self.push_scope();
            for arg in &m.signature.arguments {
                let arg_ty = self.type_from_node(&arg.type_ref);
                self.declare_value(&ident_name(&arg.ident), arg_ty);
            }
            let return_ty = self.type_from_node(&m.signature.return_type);
            self.check_block(&m.body.stmts, Some(return_ty));
            self.pop_scope();
        }
    }
}
