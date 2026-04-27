use ns_ast::{Callable, TypeModifierDecl, TypeNode};
use std::collections::{HashMap, HashSet};

use super::checker::{TypeChecker, ident_name};

impl TypeChecker<'_> {
    pub(super) fn check_type_modifier_decl(&mut self, d: &TypeModifierDecl) {
        match d {
            TypeModifierDecl::Extends(e) => {
                let Some(target_type_name) = self.type_name_from_node(&e.type_identifier) else {
                    self.report(
                        "E0627",
                        "extends type modifier requires named target type".to_string(),
                        format!("{:?}", e.type_identifier),
                        vec!["use named type in extends target".to_string()],
                    );
                    return;
                };

                let mut block_methods = HashSet::new();
                for (sig, body) in &e.methods {
                    let method_name = ident_name(&sig.ident);
                    if !block_methods.insert(method_name.clone()) {
                        self.report(
                            "E0628",
                            format!(
                                "duplicate method `{method_name}` in extends block for `{target_type_name}`"
                            ),
                            format!("extends {target_type_name} method {method_name}"),
                            vec![],
                        );
                    }

                    let existing = self.type_method_names.entry(target_type_name.clone()).or_default();
                    if !existing.insert(method_name.clone()) {
                        self.report(
                            "E0629",
                            format!("method `{method_name}` already exists for type `{target_type_name}`"),
                            format!("extends {target_type_name} method {method_name}"),
                            vec![],
                        );
                    }
                    self.type_methods
                        .entry(target_type_name.clone())
                        .or_default()
                        .insert(method_name.clone(), sig.clone());

                    self.push_scope();
                    for arg in &sig.arguments {
                        let arg_ty = self.type_from_node(&arg.type_ref);
                        self.declare_value(&ident_name(&arg.ident), arg_ty);
                    }
                    let return_ty = self.type_from_node(&sig.return_type);
                    self.check_block(&body.stmts, Some(return_ty));
                    self.pop_scope();
                }
            }
            TypeModifierDecl::Implements(i) => {
                let Some(target_type_name) = self.type_name_from_node(&i.type_identifier) else {
                    self.report(
                        "E0627",
                        "implements type modifier requires named target type".to_string(),
                        format!("{:?}", i.type_identifier),
                        vec!["use named type in implements target".to_string()],
                    );
                    return;
                };

                let mut impl_method_map: HashMap<String, Callable> = HashMap::new();
                let mut block_methods = HashSet::new();

                for (sig, _body) in &i.methods {
                    let method_name = ident_name(&sig.ident);
                    if !block_methods.insert(method_name.clone()) {
                        self.report(
                            "E0628",
                            format!(
                                "duplicate method `{method_name}` in implements block for `{target_type_name}`"
                            ),
                            format!("implements {target_type_name} method {method_name}"),
                            vec![],
                        );
                    }
                    impl_method_map.insert(method_name.clone(), sig.clone());

                    let existing = self.type_method_names.entry(target_type_name.clone()).or_default();
                    if !existing.insert(method_name.clone()) {
                        self.report(
                            "E0629",
                            format!("method `{method_name}` already exists for type `{target_type_name}`"),
                            format!("implements {target_type_name} method {method_name}"),
                            vec![],
                        );
                    }
                    self.type_methods
                        .entry(target_type_name.clone())
                        .or_default()
                        .insert(method_name.clone(), sig.clone());
                }

                for iface in &i.interfaces {
                    let iface_name = match iface {
                        TypeNode::Dynamic(dyn_ty) => ident_name(&dyn_ty.interface),
                        TypeNode::Named(named) => ident_name(&named.ident),
                    };

                    if !self.interfaces.contains_key(&iface_name) {
                        self.report(
                            "E0600",
                            format!("type modifier references non-interface `{iface_name}`"),
                            format!("{iface:?}"),
                            vec!["implements targets must be interfaces".to_string()],
                        );
                        continue;
                    }

                    if let Some(required_methods) = self.interface_methods.get(&iface_name).cloned() {
                        for (required_name, required_sig) in required_methods {
                            let actual = impl_method_map.get(&required_name);
                            let Some(actual) = actual else {
                                self.report(
                                    "E0621",
                                    format!(
                                        "type `{target_type_name}` is missing method `{required_name}` required by interface `{iface_name}` in implements block"
                                    ),
                                    format!("implements {target_type_name} for {iface_name}"),
                                    vec![],
                                );
                                continue;
                            };

                            self.check_interface_method_compatibility(
                                &target_type_name,
                                &iface_name,
                                &required_name,
                                &required_sig,
                                actual,
                            );
                        }
                    }
                }

                for (sig, body) in &i.methods {
                    self.push_scope();
                    for arg in &sig.arguments {
                        let arg_ty = self.type_from_node(&arg.type_ref);
                        self.declare_value(&ident_name(&arg.ident), arg_ty);
                    }
                    let return_ty = self.type_from_node(&sig.return_type);
                    self.check_block(&body.stmts, Some(return_ty));
                    self.pop_scope();
                }
            }
        }
    }

    fn type_name_from_node(&self, type_node: &TypeNode) -> Option<String> {
        match type_node {
            TypeNode::Named(n) => Some(ident_name(&n.ident)),
            TypeNode::Dynamic(_) => None,
        }
    }
}
