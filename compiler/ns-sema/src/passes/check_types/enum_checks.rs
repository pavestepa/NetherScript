use ns_ast::EnumDecl;
use std::collections::HashSet;

use super::checker::{TypeChecker, ident_name};

impl TypeChecker<'_> {
    pub(super) fn check_enum_decl(&mut self, e: &EnumDecl) {
        let enum_name = ident_name(&e.ident);
        self.push_scope();
        for tp in &e.type_parameters {
            self.declare_type_param(&ident_name(&tp.ident));
        }

        if let Some(interfaces) = &e.implements {
            for interface in interfaces {
                let interface_name = ident_name(interface);
                if !self.interfaces.contains_key(&interface_name) {
                    self.report(
                        "E0631",
                        format!("enum `{enum_name}` implements unknown interface `{interface_name}`"),
                        format!("enum {enum_name} implements {interface_name}"),
                        vec!["implemented targets must be interfaces".to_string()],
                    );
                    continue;
                }

                if let Some(required_methods) = self.interface_methods.get(&interface_name).cloned() {
                    for (required_name, required_sig) in required_methods {
                        let enum_method = e
                            .methods
                            .iter()
                            .find(|m| ident_name(&m.signature.ident) == required_name);

                        let Some(enum_method) = enum_method else {
                            self.report(
                                "E0632",
                                format!(
                                    "enum `{enum_name}` is missing method `{required_name}` required by interface `{interface_name}`"
                                ),
                                format!("enum {enum_name} implements {interface_name}"),
                                vec![],
                            );
                            continue;
                        };

                        self.check_interface_method_compatibility(
                            &enum_name,
                            &interface_name,
                            &required_name,
                            &required_sig,
                            &enum_method.signature,
                        );
                    }
                }
            }
        }

        let mut member_names = HashSet::new();
        for member in &e.members {
            let member_name = ident_name(&member.ident);
            if !member_names.insert(member_name.clone()) {
                self.report(
                    "E0630",
                    format!("duplicate enum member `{member_name}` in enum `{enum_name}`"),
                    format!("enum {enum_name} member {member_name}"),
                    vec![],
                );
            }
            if let Some(payload) = &member.contains {
                let _ = self.type_from_node(payload);
            }
        }

        let mut method_names = HashSet::new();
        for m in &e.methods {
            let method_name = ident_name(&m.signature.ident);
            if !method_names.insert(method_name.clone()) {
                self.report(
                    "E0633",
                    format!("duplicate method `{method_name}` in enum `{enum_name}`"),
                    format!("enum {enum_name} method {method_name}"),
                    vec![],
                );
            }

            self.push_scope();
            for tp in &e.type_parameters {
                self.declare_type_param(&ident_name(&tp.ident));
            }
            for tp in &m.signature.type_parameters {
                self.declare_type_param(&ident_name(&tp.ident));
            }
            for arg in &m.signature.arguments {
                let arg_ty = self.type_from_node(&arg.type_ref);
                self.declare_value(&ident_name(&arg.ident), arg_ty);
            }
            let return_ty = self.type_from_node(&m.signature.return_type);
            self.check_block(&m.body.stmts, Some(return_ty));
            self.pop_scope();
        }
        self.pop_scope();
    }
}
