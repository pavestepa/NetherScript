use ns_ast::{Decl, Stmt};

use super::checker::TypeChecker;
use super::types::CheckedType;

impl TypeChecker<'_> {
    pub(super) fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Const(d) => self.check_const_decl(d),
            Decl::Function(d) => self.check_function_decl(d),
            Decl::Class(c) => self.check_class_decl(c),
            Decl::Interface(i) => self.check_interface_decl(i),
            Decl::Type(_d) => {}
            Decl::Enum(e) => self.check_enum_decl(e),
            Decl::TypeModifier(d) => self.check_type_modifier_decl(d),
        }
    }

    pub(super) fn check_block(&mut self, stmts: &[Stmt], return_ty: Option<CheckedType>) {
        self.push_scope();
        for stmt in stmts {
            self.check_stmt(stmt, return_ty.clone());
        }
        self.pop_scope();
    }
}
