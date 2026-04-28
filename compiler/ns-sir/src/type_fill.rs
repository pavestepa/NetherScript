use std::collections::HashMap;

use crate::decl::{SirDecl, SirProgram};
use crate::expr::SirExpr;
use crate::ids::{SirClassId, SirTypeId, SirValueId};
use crate::stmt::SirStmt;
use crate::types::SirType;

pub fn fill_expr_types(program: &mut SirProgram) {
    let mut filler = TypeFiller::new(program);
    filler.fill();
}

struct TypeFiller<'a> {
    program: &'a mut SirProgram,
    named_type_ids: HashMap<String, SirTypeId>,
}

impl<'a> TypeFiller<'a> {
    fn new(program: &'a mut SirProgram) -> Self {
        let mut named_type_ids = HashMap::new();
        for (idx, ty) in program.types.iter().enumerate() {
            if let SirType::Named { name, args } = ty {
                if args.is_empty() {
                    named_type_ids.entry(name.clone()).or_insert(SirTypeId(idx as u32));
                }
            }
        }
        Self {
            program,
            named_type_ids,
        }
    }

    fn fill(&mut self) {
        let class_ids = self
            .program
            .classes
            .iter()
            .map(|c| (c.name.clone(), c.id))
            .collect::<HashMap<_, _>>();
        let fn_to_class = self
            .program
            .functions
            .iter()
            .map(|f| (f.id, self.class_of_method(f.id)))
            .collect::<HashMap<_, _>>();
        let decl_len = self.program.decls.len();
        for idx in 0..decl_len {
            let mut decl_tmp = std::mem::replace(&mut self.program.decls[idx], SirDecl::Error);
            match &mut decl_tmp {
                SirDecl::Const { expr, .. } => {
                    self.fill_expr(expr, None, &class_ids);
                }
                SirDecl::Function { id, body } => {
                    let this_class = fn_to_class.get(id).copied().unwrap_or(None);
                    for stmt in body {
                        self.fill_stmt(stmt, this_class, &class_ids);
                    }
                }
                _ => {}
            }
            self.program.decls[idx] = decl_tmp;
        }
    }

    fn fill_stmt(
        &mut self,
        stmt: &mut SirStmt,
        this_class: Option<SirClassId>,
        class_ids: &HashMap<String, SirClassId>,
    ) {
        match stmt {
            SirStmt::Expr(expr) => self.fill_expr(expr, this_class, class_ids),
            SirStmt::Let { value, init, .. } => {
                if let Some(expr) = init {
                    self.fill_expr(expr, this_class, class_ids);
                    if let Some(init_ty) = expr.ty() {
                        if let Some(symbol) = self
                            .program
                            .values
                            .iter_mut()
                            .find(|sym| sym.id == *value)
                        {
                            if symbol.ty.is_none() {
                                symbol.ty = Some(init_ty);
                            }
                        }
                    }
                }
            }
            SirStmt::Assign { target, value } => {
                self.fill_expr(target, this_class, class_ids);
                self.fill_expr(value, this_class, class_ids);
            }
            SirStmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.fill_expr(expr, this_class, class_ids);
                }
            }
            SirStmt::If {
                test,
                then_body,
                else_body,
            } => {
                self.fill_expr(test, this_class, class_ids);
                for nested in then_body {
                    self.fill_stmt(nested, this_class, class_ids);
                }
                for nested in else_body {
                    self.fill_stmt(nested, this_class, class_ids);
                }
            }
            SirStmt::Loop { body } => {
                for nested in body {
                    self.fill_stmt(nested, this_class, class_ids);
                }
            }
            SirStmt::Error | SirStmt::InitParent { .. } | SirStmt::InitField { .. } | SirStmt::Break => {}
        }
    }

    fn fill_expr(
        &mut self,
        expr: &mut SirExpr,
        this_class: Option<SirClassId>,
        class_ids: &HashMap<String, SirClassId>,
    ) {
        match expr {
            SirExpr::Error { .. } => {}
            SirExpr::Literal { ty, lit } => {
                if ty.is_none() {
                    *ty = Some(match lit {
                        crate::expr::SirLiteral::Number(_) => self.named("i32"),
                        crate::expr::SirLiteral::String(_) => self.named("string"),
                        crate::expr::SirLiteral::Boolean(_) => self.named("bool"),
                    });
                }
            }
            SirExpr::ValueRef { value, ty } => {
                if ty.is_none() {
                    *ty = self.value_type(*value).or_else(|| this_class.map(|id| self.class_type(id)));
                }
                if ty.is_none() {
                    *ty = Some(self.error_type());
                }
            }
            SirExpr::TypeRef { type_id, ty } => {
                if ty.is_none() {
                    *ty = Some(*type_id);
                }
            }
            SirExpr::Call {
                callee,
                type_args,
                args,
                ty,
            } => {
                for arg in args {
                    self.fill_expr(arg, this_class, class_ids);
                }
                if ty.is_none() {
                    let inferred = callee
                        .and_then(|id| self.function_ret_type(id))
                        .map(|ret| self.instantiate_call_return_type(ret, type_args));
                    *ty = inferred.or(Some(self.error_type()));
                }
            }
            SirExpr::IntrinsicPrintln { args, ty } => {
                for arg in args {
                    self.fill_expr(arg, this_class, class_ids);
                }
                if ty.is_none() {
                    *ty = Some(self.named("void"));
                }
            }
            SirExpr::TemplateString { parts, ty } => {
                for part in parts {
                    if let crate::expr::SirTemplatePart::Expr(expr) = part {
                        self.fill_expr(expr, this_class, class_ids);
                    }
                }
                if ty.is_none() {
                    *ty = Some(self.named("string"));
                }
            }
            SirExpr::Unary { op, value, ty } => {
                self.fill_expr(value, this_class, class_ids);
                if ty.is_none() {
                    *ty = if op == "!" {
                        Some(self.named("bool"))
                    } else {
                        value.ty().or(Some(self.error_type()))
                    };
                }
            }
            SirExpr::Binary {
                left, right, ty, ..
            } => {
                self.fill_expr(left, this_class, class_ids);
                self.fill_expr(right, this_class, class_ids);
                if ty.is_none() {
                    *ty = left.ty().or(right.ty()).or(Some(self.error_type()));
                }
            }
            SirExpr::Member {
                object,
                property,
                ty,
            } => {
                self.fill_expr(object, this_class, class_ids);
                if ty.is_none() {
                    *ty = object
                        .ty()
                        .and_then(|obj_ty| self.field_type_by_member(obj_ty, property))
                        .or(Some(self.error_type()));
                }
            }
            SirExpr::StructLiteral {
                type_name,
                fields,
                ty,
            } => {
                for (_, value) in fields {
                    self.fill_expr(value, this_class, class_ids);
                }
                if ty.is_none() {
                    *ty = Some(self.named(type_name));
                }
            }
            SirExpr::New {
                class_name,
                args,
                ty,
            } => {
                for arg in args {
                    self.fill_expr(arg, this_class, class_ids);
                }
                if ty.is_none() {
                    *ty = class_ids
                        .get(class_name)
                        .copied()
                        .map(|id| self.class_type(id))
                        .or(Some(self.error_type()));
                }
            }
        }
    }

    fn class_of_method(&self, fn_id: crate::ids::SirFnId) -> Option<SirClassId> {
        self.program
            .functions
            .iter()
            .find(|f| f.id == fn_id)
            .and_then(|f| f.name.split_once("::").map(|(name, _)| name))
            .and_then(|class_name| self.program.classes.iter().find(|c| c.name == class_name))
            .map(|c| c.id)
    }

    fn named(&mut self, name: &str) -> SirTypeId {
        if let Some(id) = self.named_type_ids.get(name) {
            return *id;
        }
        let id = SirTypeId(self.program.types.len() as u32);
        self.program.types.push(SirType::Named {
            name: name.to_string(),
            args: Vec::new(),
        });
        self.named_type_ids.insert(name.to_string(), id);
        id
    }

    fn class_type(&mut self, class_id: SirClassId) -> SirTypeId {
        let class_name = self
            .program
            .classes
            .iter()
            .find(|c| c.id == class_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "unknown_class".to_string());
        self.named(&class_name)
    }

    fn error_type(&self) -> SirTypeId {
        SirTypeId(0)
    }

    fn value_type(&self, value_id: SirValueId) -> Option<SirTypeId> {
        self.program
            .values
            .iter()
            .find(|v| v.id == value_id)
            .and_then(|v| v.ty)
    }

    fn function_ret_type(&self, fn_id: crate::ids::SirFnId) -> Option<SirTypeId> {
        self.program
            .functions
            .iter()
            .find(|f| f.id == fn_id)
            .and_then(|f| f.ret)
    }

    fn field_type_by_member(&self, obj_ty: SirTypeId, property: &str) -> Option<SirTypeId> {
        let class_name = match self.program.types.get(obj_ty.0 as usize)? {
            SirType::Named { name, .. } => name.as_str(),
            _ => return None,
        };
        let class = self.program.classes.iter().find(|c| c.name == class_name)?;
        let field = self
            .program
            .fields
            .iter()
            .find(|f| f.owner == class.id && f.name == property)?;
        field.ty
    }

    fn instantiate_call_return_type(
        &self,
        return_ty: SirTypeId,
        type_args: &[SirTypeId],
    ) -> SirTypeId {
        let Some(ty) = self.program.types.get(return_ty.0 as usize) else {
            return return_ty;
        };
        match ty {
            SirType::TypeParam { .. } if type_args.len() == 1 => type_args[0],
            SirType::Named { name, args }
                if args.is_empty()
                    && type_args.len() == 1
                    && name.len() == 1
                    && name.chars().all(|c| c.is_ascii_uppercase()) =>
            {
                type_args[0]
            }
            _ => return_ty,
        }
    }
}
