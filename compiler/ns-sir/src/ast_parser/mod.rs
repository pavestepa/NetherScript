use std::collections::{HashMap, HashSet};

use ns_ast::{Decl, Expr, Module, Stmt, TypeNode};

use crate::decl::SirProgram;
use crate::diagnostics::{SirResult, sir_error};
use crate::expr::SirExpr;
use crate::ids::{SirClassId, SirEnumId, SirFnId, SirTraitId, SirTypeId, SirValueId};
use crate::stmt::SirStmt;
use crate::symbols::{SirFunctionSymbol, SirValueSymbol};
use crate::types::SirType;

mod decl;
mod expr;
mod module;
mod shared;
mod stmt;
mod type_node;

pub fn lower_ast_module(module: &Module) -> SirResult<SirProgram> {
    let mut lowerer = Lowerer::default();
    module::lower_module(&mut lowerer, module)?;
    Ok(lowerer.finish())
}

#[derive(Default)]
pub(crate) struct Lowerer {
    pub(crate) program: SirProgram,
    pub(crate) value_scopes: Vec<HashMap<String, SirValueId>>,
    pub(crate) type_param_scopes: Vec<HashSet<String>>,
    pub(crate) class_by_name: HashMap<String, SirClassId>,
    pub(crate) trait_by_class_name: HashMap<String, SirTraitId>,
    pub(crate) enum_by_name: HashMap<String, SirEnumId>,
    pub(crate) next_value_id: u32,
    pub(crate) next_fn_id: u32,
    pub(crate) next_class_id: u32,
    pub(crate) next_field_id: u32,
    pub(crate) next_trait_id: u32,
    pub(crate) next_enum_id: u32,
    pub(crate) interned_types: HashMap<SirTypeKey, SirTypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum SirTypeKey {
    Error,
    Named { name: String, args: Vec<SirTypeId> },
    Dynamic { interface: String },
    TypeParam { name: String },
}

impl Lowerer {
    pub(crate) fn lower_decl(&mut self, decl: &Decl) -> SirResult<()> {
        decl::lower_decl(self, decl)
    }

    pub(crate) fn lower_stmt(&mut self, stmt: &Stmt) -> SirStmt {
        stmt::lower_stmt(self, stmt)
    }

    pub(crate) fn lower_expr(&mut self, expr: &Expr) -> SirExpr {
        expr::lower_expr(self, expr)
    }

    pub(crate) fn lower_type(&mut self, node: &TypeNode) -> SirTypeId {
        type_node::lower_type(self, node)
    }

    pub(crate) fn alloc_value(&mut self, name: &str, ty: Option<SirTypeId>) -> SirValueId {
        let id = SirValueId(self.next_value_id);
        self.next_value_id += 1;
        self.program.values.push(SirValueSymbol {
            id,
            name: name.to_string(),
            ty,
        });
        id
    }

    pub(crate) fn declare_value(&mut self, name: String, id: SirValueId) {
        if let Some(scope) = self.value_scopes.last_mut() {
            scope.insert(name, id);
        }
    }

    pub(crate) fn lookup_value(&self, name: &str) -> Option<SirValueId> {
        for scope in self.value_scopes.iter().rev() {
            if let Some(id) = scope.get(name) {
                return Some(*id);
            }
        }
        None
    }

    pub(crate) fn find_function_id(&self, name: &str) -> Option<SirFnId> {
        self.program
            .functions
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.id)
    }

    pub(crate) fn function_symbol_mut(&mut self, fn_id: SirFnId) -> Option<&mut SirFunctionSymbol> {
        self.program.functions.iter_mut().find(|f| f.id == fn_id)
    }

    pub(crate) fn alloc_fn_symbol(
        &mut self,
        name: String,
        params: Vec<SirValueId>,
        ret: Option<SirTypeId>,
    ) -> SirFnId {
        let id = SirFnId(self.next_fn_id);
        self.next_fn_id += 1;
        self.program.functions.push(SirFunctionSymbol { id, name, params, ret });
        id
    }

    pub(crate) fn class_name(&self, class_id: SirClassId) -> Option<&str> {
        self.program
            .classes
            .iter()
            .find(|c| c.id == class_id)
            .map(|c| c.name.as_str())
    }

    pub(crate) fn named_type(&mut self, name: &str) -> Option<SirTypeId> {
        Some(self.intern_type(SirType::Named {
            name: name.to_string(),
            args: Vec::new(),
        }))
    }

    pub(crate) fn bool_type(&mut self) -> Option<SirTypeId> {
        self.named_type("bool")
    }

    pub(crate) fn value_type(&self, value_id: SirValueId) -> Option<SirTypeId> {
        self.program
            .values
            .iter()
            .find(|v| v.id == value_id)
            .and_then(|v| v.ty)
    }

    pub(crate) fn function_ret_type(&self, fn_id: SirFnId) -> Option<SirTypeId> {
        self.program
            .functions
            .iter()
            .find(|f| f.id == fn_id)
            .and_then(|f| f.ret)
    }

    pub(crate) fn intern_type(&mut self, ty: SirType) -> SirTypeId {
        let key = SirTypeKey::from(&ty);
        if let Some(existing) = self.interned_types.get(&key) {
            return *existing;
        }
        let id = SirTypeId(self.program.types.len() as u32);
        self.program.types.push(ty);
        self.interned_types.insert(key, id);
        id
    }

    pub(crate) fn push_scope(&mut self) {
        self.value_scopes.push(HashMap::new());
    }

    pub(crate) fn pop_scope(&mut self) {
        self.value_scopes.pop();
    }

    pub(crate) fn push_type_param_scope(&mut self) {
        self.type_param_scopes.push(HashSet::new());
    }

    pub(crate) fn pop_type_param_scope(&mut self) {
        self.type_param_scopes.pop();
    }

    pub(crate) fn declare_type_param(&mut self, name: &str) {
        if let Some(scope) = self.type_param_scopes.last_mut() {
            scope.insert(name.to_string());
        }
    }

    pub(crate) fn is_type_param(&self, name: &str) -> bool {
        self.type_param_scopes
            .iter()
            .rev()
            .any(|scope| scope.contains(name))
    }

    fn finish(self) -> SirProgram {
        self.program
    }
}

impl From<&SirType> for SirTypeKey {
    fn from(value: &SirType) -> Self {
        match value {
            SirType::Error => SirTypeKey::Error,
            SirType::Named { name, args } => SirTypeKey::Named {
                name: name.clone(),
                args: args.clone(),
            },
            SirType::Dynamic { interface } => SirTypeKey::Dynamic {
                interface: interface.clone(),
            },
            SirType::TypeParam { name } => SirTypeKey::TypeParam { name: name.clone() },
        }
    }
}

impl Default for SirProgram {
    fn default() -> Self {
        Self {
            decls: Vec::new(),
            values: Vec::new(),
            functions: Vec::new(),
            classes: Vec::new(),
            enums: Vec::new(),
            fields: Vec::new(),
            traits: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            types: vec![SirType::Error],
        }
    }
}

pub(crate) fn missing_predeclared_class_err(name: &str) -> Vec<ns_diagnostics::Diagnostic> {
    vec![sir_error("SIR1001", format!("missing predeclared class `{name}`"))]
}
