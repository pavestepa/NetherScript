use ns_ast::TypeNode;

use crate::types::{Type, TypeId};

use super::{checker::TypeChecker, types::CheckedType};

impl TypeChecker<'_> {
    pub(super) fn type_from_node(&mut self, type_node: &TypeNode) -> CheckedType {
        let type_id = self.intern_type_node(type_node);
        match type_node {
            TypeNode::Named(n) => {
                let name = super::checker::ident_name(&n.ident);
                if self.lookup_type_param(&name).is_some()
                    || self.builtin_checked_by_name(&name).is_some()
                    || self.global_types.contains_key(&name)
                {
                    CheckedType::Resolved(type_id)
                } else {
                    self.report(
                        "E0634",
                        format!("unknown type in type checker environment: `{name}`"),
                        format!("{type_node:?}"),
                        vec!["type should be declared in global type namespace".to_string()],
                    );
                    CheckedType::Error
                }
            }
            TypeNode::Dynamic(d) => {
                let iface = super::checker::ident_name(&d.interface);
                if !self.interfaces.contains_key(&iface) {
                    // E0612: dynamic type must point to a known interface declaration.
                    self.report(
                        "E0612",
                        format!("dynamic type must reference interface, found `{iface}`"),
                        format!("{type_node:?}"),
                        vec![],
                    );
                }
                CheckedType::Resolved(type_id)
            }
        }
    }

    pub(super) fn intern_type_node(&mut self, type_node: &TypeNode) -> TypeId {
        match type_node {
            TypeNode::Named(n) => {
                let name = super::checker::ident_name(&n.ident);
                if n.type_arguments.is_empty() {
                    if let Some(type_param) = self.lookup_type_param(&name) {
                        return type_param;
                    }
                    if let Some(builtin) = self.builtin_type_id_by_name(&name) {
                        return builtin;
                    }
                    if let Some(global) = self.global_types.get(&name) {
                        return *global;
                    }
                    return self.intern_named_leaf(&name);
                }
                let mut args = Vec::with_capacity(n.type_arguments.len());
                for arg in &n.type_arguments {
                    args.push(self.intern_type_node(arg));
                }
                let key = (name.clone(), args.clone());
                if let Some(existing) = self.canonical_named_types.get(&key) {
                    return *existing;
                }
                let id = self.ctx.intern_type(Type::Named { name, args });
                self.canonical_named_types.insert(key, id);
                id
            }
            TypeNode::Dynamic(d) => self.ctx.intern_type(Type::Dynamic {
                interface: super::checker::ident_name(&d.interface),
            }),
        }
    }

    pub(super) fn is_assignable(&self, expected: &CheckedType, got: &CheckedType) -> bool {
        match (expected, got) {
            (CheckedType::Error, _) | (_, CheckedType::Error) => true,
            (CheckedType::Resolved(exp), CheckedType::Resolved(actual)) => {
                if exp == actual {
                    return true;
                }
                if *actual == self.builtins.never {
                    return true;
                }
                let exp_name = self.type_leaf_name(*exp);
                let actual_name = self.type_leaf_name(*actual);
                match (exp_name, actual_name) {
                    (Some(e), Some(a)) => a == e || self.is_subclass_of(&a, &e),
                    _ => false,
                }
            }
            (CheckedType::Callable { params: ep, ret: er }, CheckedType::Callable { params: ap, ret: ar }) => {
                if ep.len() != ap.len() {
                    return false;
                }
                if er != ar {
                    return false;
                }
                ep.iter().zip(ap.iter()).all(|(e, a)| e == a)
            }
            (CheckedType::Callable { .. }, _) | (_, CheckedType::Callable { .. }) => false,
        }
    }

    pub(super) fn is_subclass_of(&self, class: &str, target_parent: &str) -> bool {
        let mut cur = class;
        while let Some(parent) = self.class_parent.get(cur) {
            if parent == target_parent {
                return true;
            }
            cur = parent;
        }
        false
    }

    fn builtin_checked_by_name(&self, name: &str) -> Option<CheckedType> {
        self.builtins.get_by_name(name).map(CheckedType::Resolved)
    }

    fn builtin_type_id_by_name(&self, name: &str) -> Option<TypeId> {
        self.builtins.get_by_name(name)
    }

    pub(super) fn type_leaf_name(&self, id: TypeId) -> Option<String> {
        let ty = self.ctx.types.get(id.0 as usize)?;
        match ty {
            Type::Named { name, .. } => Some(name.clone()),
            Type::Dynamic { interface } => Some(interface.clone()),
            Type::TypeParam { name } => Some(name.clone()),
            Type::Error => None,
        }
    }
}
