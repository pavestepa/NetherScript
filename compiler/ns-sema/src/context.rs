use std::collections::HashMap;

use ns_ast::Module;

use crate::{
    builtins::{BUILTIN_TYPE_CATALOG, BuiltinTypeKey},
    diagnostic::Diagnostic,
    passes,
    scope::Scope,
    symbol::{Symbol, SymbolId, SymbolKind},
    types::{Type, TypeId},
};

pub type SemaResult<T> = Result<T, Vec<Diagnostic>>;

#[derive(Debug, Default)]
pub struct SemaContext {
    pub diagnostics: Vec<Diagnostic>,
    pub symbols: Vec<Symbol>,
    pub scopes: Vec<Scope>,
    pub current_scope: usize,
    pub types: Vec<Type>,
    pub builtins: Option<BuiltinTypes>,
}

#[derive(Debug, Clone)]
pub struct BuiltinTypes {
    pub boolean: TypeId,
    pub string: TypeId,
    pub void: TypeId,
    pub never: TypeId,
    pub i32: TypeId,
    pub u32: TypeId,
    pub f32: TypeId,
    pub f64: TypeId,
    pub error: TypeId,
    by_key: HashMap<BuiltinTypeKey, TypeId>,
    by_name: HashMap<String, TypeId>,
}

impl BuiltinTypes {
    pub fn get(&self, key: BuiltinTypeKey) -> Option<TypeId> {
        self.by_key.get(&key).copied()
    }

    pub fn get_by_name(&self, name: &str) -> Option<TypeId> {
        self.by_name.get(name).copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AnalyzeOptions {
    pub use_prelude: bool,
}

impl Default for AnalyzeOptions {
    fn default() -> Self {
        Self { use_prelude: true }
    }
}

impl SemaContext {
    pub fn new() -> Self {
        let mut ctx = Self::default();
        ctx.scopes.push(Scope::default());
        ctx
    }

    pub fn push_scope(&mut self) {
        let parent = self.current_scope;
        self.scopes.push(Scope {
            parent: Some(parent),
            ..Scope::default()
        });
        self.current_scope = self.scopes.len() - 1;
    }

    pub fn pop_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    pub fn intern_symbol(&mut self, name: String, kind: SymbolKind) -> SymbolId {
        let id = SymbolId(self.symbols.len() as u32);
        self.symbols.push(Symbol { id, name, kind });
        id
    }

    pub fn declare_type_in_current_scope(&mut self, name: &str, id: SymbolId) -> bool {
        let scope = &mut self.scopes[self.current_scope];
        if scope.type_names.contains_key(name) {
            return false;
        }
        scope.type_names.insert(name.to_string(), id);
        true
    }

    pub fn declare_value_in_current_scope(&mut self, name: &str, id: SymbolId) -> bool {
        let scope = &mut self.scopes[self.current_scope];
        if scope.value_names.contains_key(name) {
            return false;
        }
        scope.value_names.insert(name.to_string(), id);
        true
    }

    pub fn resolve_type_name(&self, name: &str) -> Option<SymbolId> {
        let mut scope_idx = Some(self.current_scope);
        while let Some(idx) = scope_idx {
            if let Some(id) = self.scopes[idx].type_names.get(name) {
                return Some(*id);
            }
            scope_idx = self.scopes[idx].parent;
        }
        None
    }

    pub fn resolve_value_name(&self, name: &str) -> Option<SymbolId> {
        let mut scope_idx = Some(self.current_scope);
        while let Some(idx) = scope_idx {
            if let Some(id) = self.scopes[idx].value_names.get(name) {
                return Some(*id);
            }
            scope_idx = self.scopes[idx].parent;
        }
        None
    }

    pub fn intern_type(&mut self, ty: Type) -> TypeId {
        let id = TypeId(self.types.len() as u32);
        self.types.push(ty);
        id
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::error(message));
    }

    pub fn finish(self) -> SemaResult<Self> {
        if self.diagnostics.is_empty() {
            Ok(self)
        } else {
            Err(self.diagnostics)
        }
    }
}

pub fn analyze(module: &Module) -> SemaResult<SemaContext> {
    analyze_with_options(module, AnalyzeOptions::default())
}

pub fn analyze_with_options(module: &Module, options: AnalyzeOptions) -> SemaResult<SemaContext> {
    let mut ctx = SemaContext::new();
    seed_builtins(&mut ctx);
    if options.use_prelude {
        seed_prelude(&mut ctx);
    }
    passes::collect_decls::run(&mut ctx, module);
    passes::resolve_names::run(&mut ctx, module);
    passes::check_types::run(&mut ctx, module);
    ctx.finish()
}

pub fn seed_builtins(ctx: &mut SemaContext) {
    let mut by_key = HashMap::new();
    let mut by_name = HashMap::new();

    for spec in BUILTIN_TYPE_CATALOG {
        let ty = ctx.intern_type(Type::Named {
            name: spec.name.to_string(),
            args: Vec::new(),
        });
        by_key.insert(spec.key, ty);
        by_name.insert(spec.name.to_string(), ty);
        declare_builtin_type(ctx, spec.name);
    }

    let error = ctx.intern_type(Type::Error);

    let boolean = by_key[&BuiltinTypeKey::Boolean];
    let string = by_key[&BuiltinTypeKey::String];
    let void = by_key[&BuiltinTypeKey::Void];
    let never = by_key[&BuiltinTypeKey::Never];
    let i32 = by_key[&BuiltinTypeKey::I32];
    let u32 = by_key[&BuiltinTypeKey::U32];
    let f32 = by_key[&BuiltinTypeKey::F32];
    let f64 = by_key[&BuiltinTypeKey::F64];

    ctx.builtins = Some(BuiltinTypes {
        boolean,
        string,
        void,
        never,
        i32,
        u32,
        f32,
        f64,
        error,
        by_key,
        by_name,
    });
}

pub fn seed_prelude(ctx: &mut SemaContext) {
    // Virtual `std::prelude`: available to all modules unless explicitly disabled.
    declare_prelude_type(ctx, "Any");
    declare_prelude_type(ctx, "Number");
    declare_prelude_type(ctx, "ToString");
    declare_prelude_type(ctx, "Debug");

    declare_prelude_value(ctx, "print");
    declare_prelude_value(ctx, "panic");
}

fn declare_builtin_type(ctx: &mut SemaContext, name: &str) {
    let sid = ctx.intern_symbol(name.to_string(), SymbolKind::Type);
    if !ctx.declare_type_in_current_scope(name, sid) {
        ctx.error(format!("duplicate builtin type in global scope: {name}"));
    }
}

fn declare_prelude_type(ctx: &mut SemaContext, name: &str) {
    let sid = ctx.intern_symbol(name.to_string(), SymbolKind::Type);
    if !ctx.declare_type_in_current_scope(name, sid) {
        ctx.error(format!("duplicate prelude type in global scope: {name}"));
    }
}

fn declare_prelude_value(ctx: &mut SemaContext, name: &str) {
    let sid = ctx.intern_symbol(name.to_string(), SymbolKind::Value);
    if !ctx.declare_value_in_current_scope(name, sid) {
        ctx.error(format!("duplicate prelude value in global scope: {name}"));
    }
}
