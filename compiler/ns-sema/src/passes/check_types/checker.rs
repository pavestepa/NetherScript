use std::collections::{HashMap, HashSet};

use ns_ast::{Callable, Decl, Ident, Module};

use crate::context::{BuiltinTypes, SemaContext};
use crate::types::{Type, TypeId};

use super::types::CheckedType;

pub(super) struct TypeChecker<'a> {
    pub(super) ctx: &'a mut SemaContext,
    pub(super) builtins: BuiltinTypes,
    pub(super) class_parent: HashMap<String, String>,
    pub(super) class_names: HashSet<String>,
    pub(super) interfaces: HashMap<String, ()>,
    pub(super) interface_methods: HashMap<String, HashMap<String, Callable>>,
    pub(super) type_method_names: HashMap<String, HashSet<String>>,
    pub(super) type_methods: HashMap<String, HashMap<String, Callable>>,
    pub(super) function_type_params: HashMap<String, Vec<String>>,
    pub(super) type_fields: HashMap<String, HashMap<String, Option<TypeId>>>,
    pub(super) global_values: HashMap<String, CheckedType>,
    pub(super) global_types: HashMap<String, TypeId>,
    pub(super) value_scopes: Vec<HashMap<String, CheckedType>>,
    pub(super) init_scopes: Vec<HashMap<String, bool>>,
    pub(super) type_scopes: Vec<HashMap<String, TypeId>>,
    pub(super) named_type_cache: HashMap<String, TypeId>,
    pub(super) canonical_named_types: HashMap<(String, Vec<TypeId>), TypeId>,
}

impl<'a> TypeChecker<'a> {
    pub(super) fn new(ctx: &'a mut SemaContext, builtins: BuiltinTypes, module: &Module) -> Self {
        let mut class_parent = HashMap::new();
        let mut class_names = HashSet::new();
        let mut interfaces = HashMap::new();
        let mut interface_methods: HashMap<String, HashMap<String, Callable>> = HashMap::new();
        let mut type_method_names: HashMap<String, HashSet<String>> = HashMap::new();
        let mut type_methods: HashMap<String, HashMap<String, Callable>> = HashMap::new();
        let mut function_type_params: HashMap<String, Vec<String>> = HashMap::new();

        for module_decl in module.decls() {
            match module_decl.decl() {
                Decl::Class(c) => {
                    let class_name = ident_name(&c.ident);
                    class_names.insert(class_name.clone());
                    let mut methods = HashSet::new();
                    for m in &c.methods {
                        methods.insert(ident_name(&m.signature.ident));
                    }
                    type_method_names.insert(class_name.clone(), methods);
                    let mut method_sigs = HashMap::new();
                    for m in &c.methods {
                        method_sigs.insert(ident_name(&m.signature.ident), m.signature.clone());
                    }
                    type_methods.insert(class_name.clone(), method_sigs);
                    if let Some(parent) = &c.extends {
                        class_parent.insert(class_name, ident_name(parent));
                    }
                }
                Decl::Enum(e) => {
                    let enum_name = ident_name(&e.ident);
                    let mut methods = HashSet::new();
                    for m in &e.methods {
                        methods.insert(ident_name(&m.signature.ident));
                    }
                    type_method_names.insert(enum_name, methods);
                    let enum_name2 = ident_name(&e.ident);
                    let mut method_sigs = HashMap::new();
                    for m in &e.methods {
                        method_sigs.insert(ident_name(&m.signature.ident), m.signature.clone());
                    }
                    type_methods.insert(enum_name2, method_sigs);
                }
                Decl::Interface(i) => {
                    let interface_name = ident_name(&i.ident);
                    interfaces.insert(interface_name.clone(), ());
                    let mut methods = HashMap::new();
                    for (callable, _body) in &i.methods {
                        methods.insert(ident_name(&callable.ident), callable.clone());
                    }
                    interface_methods.insert(interface_name, methods);
                }
                Decl::Function(f) => {
                    function_type_params.insert(
                        ident_name(&f.signature.ident),
                        f.signature
                            .type_parameters
                            .iter()
                            .map(|tp| ident_name(&tp.ident))
                            .collect(),
                    );
                }
                _ => {}
            }
        }

        let mut checker = Self {
            ctx,
            builtins,
            class_parent,
            class_names,
            interfaces,
            interface_methods,
            type_method_names,
            type_methods,
            function_type_params,
            type_fields: HashMap::new(),
            global_values: HashMap::new(),
            global_types: HashMap::new(),
            value_scopes: vec![HashMap::new()],
            init_scopes: vec![HashMap::new()],
            type_scopes: vec![HashMap::new()],
            named_type_cache: HashMap::new(),
            canonical_named_types: HashMap::new(),
        };
        checker.collect_global_environments(module);
        checker
    }

    pub(super) fn check_module(&mut self, module: &Module) {
        self.validate_class_hierarchy();
        for module_decl in module.decls() {
            self.check_decl(module_decl.decl());
        }
    }

    pub(super) fn push_scope(&mut self) {
        self.value_scopes.push(HashMap::new());
        self.init_scopes.push(HashMap::new());
        self.type_scopes.push(HashMap::new());
    }

    pub(super) fn pop_scope(&mut self) {
        self.value_scopes.pop();
        self.init_scopes.pop();
        self.type_scopes.pop();
        if self.value_scopes.is_empty() {
            self.value_scopes.push(HashMap::new());
        }
        if self.init_scopes.is_empty() {
            self.init_scopes.push(HashMap::new());
        }
        if self.type_scopes.is_empty() {
            self.type_scopes.push(HashMap::new());
        }
    }

    pub(super) fn declare_value(&mut self, name: &str, ty: CheckedType) {
        if let Some(scope) = self.value_scopes.last_mut() {
            scope.insert(name.to_string(), ty);
        }
    }

    pub(super) fn declare_value_init(&mut self, name: &str, initialized: bool) {
        if let Some(scope) = self.init_scopes.last_mut() {
            scope.insert(name.to_string(), initialized);
        }
    }

    pub(super) fn mark_initialized(&mut self, name: &str) -> bool {
        for scope in self.init_scopes.iter_mut().rev() {
            if let Some(init) = scope.get_mut(name) {
                *init = true;
                return true;
            }
        }
        false
    }

    pub(super) fn is_definitely_initialized(&self, name: &str) -> Option<bool> {
        for scope in self.init_scopes.iter().rev() {
            if let Some(init) = scope.get(name) {
                return Some(*init);
            }
        }
        if self.global_values.contains_key(name) {
            return Some(true);
        }
        None
    }

    pub(super) fn lookup_value(&self, name: &str) -> Option<CheckedType> {
        for scope in self.value_scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        self.global_values.get(name).cloned()
    }

    pub(super) fn declare_type_param(&mut self, name: &str) -> TypeId {
        let tid = self.ctx.intern_type(Type::TypeParam {
            name: name.to_string(),
        });
        if let Some(scope) = self.type_scopes.last_mut() {
            scope.insert(name.to_string(), tid);
        }
        tid
    }

    pub(super) fn lookup_type_param(&self, name: &str) -> Option<TypeId> {
        for scope in self.type_scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(*ty);
            }
        }
        None
    }

    fn collect_global_environments(&mut self, module: &Module) {
        for module_decl in module.decls() {
            match module_decl.decl() {
                Decl::Const(c) => {
                    let value_name = ident_name(&c.binding.ident);
                    let value_ty = self.type_from_node(&c.binding.type_ref);
                    self.global_values.insert(value_name, value_ty);
                }
                Decl::Function(f) => {
                    let value_name = ident_name(&f.signature.ident);
                    let callable_ty = self.callable_type_from_signature(&f.signature);
                    self.global_values.insert(value_name, callable_ty);
                }
                Decl::Class(c) => {
                    let name = ident_name(&c.ident);
                    let ty = self.intern_named_leaf(&name);
                    self.global_types.insert(name, ty);
                    let mut fields = HashMap::new();
                    for field in &c.fields {
                        let fname = ident_name(&field.binding.ident);
                        let fty = field.binding.type_ref.as_ref().map(|t| self.intern_type_node(t));
                        fields.insert(fname, fty);
                    }
                    self.type_fields.insert(ident_name(&c.ident), fields);
                }
                Decl::Interface(i) => {
                    let name = ident_name(&i.ident);
                    let ty = self.intern_named_leaf(&name);
                    self.global_types.insert(name, ty);
                }
                Decl::Enum(e) => {
                    let name = ident_name(&e.ident);
                    let ty = self.intern_named_leaf(&name);
                    self.global_types.insert(name, ty);
                }
                Decl::Type(t) => {
                    let type_name = ident_name(&t.ident);
                    let type_ty = self.intern_type_node(&t.assign);
                    self.global_types.insert(type_name, type_ty);
                }
                Decl::TypeModifier(_) => {}
            }
        }
    }

    fn validate_class_hierarchy(&mut self) {
        let mut pending_reports: Vec<(String, String, String, Vec<String>)> = Vec::new();

        // E0616: every `extends` target must be a declared class.
        for (class, parent) in &self.class_parent {
            if !self.class_names.contains(parent) {
                pending_reports.push((
                    "E0616".to_string(),
                    format!("class `{class}` extends unknown class `{parent}`"),
                    format!("class {class} extends {parent}"),
                    vec!["base type in `extends` must be a class declaration".to_string()],
                ));
            }
        }

        // E0620: detect inheritance cycles to keep assignability/codegen sound.
        for class in &self.class_names {
            let mut seen = HashSet::new();
            let mut cur = class.as_str();
            while let Some(parent) = self.class_parent.get(cur) {
                if !seen.insert(cur.to_string()) {
                    pending_reports.push((
                        "E0620".to_string(),
                        format!("cyclic inheritance detected starting at `{class}`"),
                        format!("class hierarchy around `{class}`"),
                        vec!["remove cycle in extends chain".to_string()],
                    ));
                    break;
                }
                cur = parent;
            }
        }

        for (code, message, snippet, notes) in pending_reports {
            self.report(&code, message, snippet, notes);
        }
    }

    pub(super) fn intern_named_leaf(&mut self, name: &str) -> TypeId {
        if let Some(existing) = self.named_type_cache.get(name) {
            return *existing;
        }
        let tid = self.ctx.intern_type(Type::Named {
            name: name.to_string(),
            args: Vec::new(),
        });
        self.named_type_cache.insert(name.to_string(), tid);
        tid
    }

    pub(super) fn callable_type_from_signature(&mut self, sig: &Callable) -> CheckedType {
        self.push_scope();
        for tp in &sig.type_parameters {
            self.declare_type_param(&ident_name(&tp.ident));
        }
        let mut params = Vec::with_capacity(sig.arguments.len());
        for arg in &sig.arguments {
            params.push(self.intern_type_node(&arg.type_ref));
        }
        let ret = self.intern_type_node(&sig.return_type);
        self.pop_scope();
        CheckedType::Callable { params, ret }
    }
}

pub(super) fn ident_name(ident: &Ident) -> String {
    ident.clone().into_simple().as_str().to_string()
}
