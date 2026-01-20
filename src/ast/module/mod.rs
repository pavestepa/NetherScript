use crate::ast::{ast::Ast, decl::Decl};

#[derive(Debug)]
pub struct Module {
    decls: Vec<Ast<Decl>>,
}

impl Module {
    pub fn new(decls: Vec<Ast<Decl>>) -> Self {
        Self { decls }
    }

    pub fn decls(&self) -> &[Ast<Decl>] {
        &self.decls
    }
}
