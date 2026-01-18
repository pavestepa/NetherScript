use crate::ast::decl::Decl;

#[derive(Debug)]
pub struct Module {
    decls: Vec<Ast<Decl>>,
}

impl Module {
    pub fn new(decls: Vec<Decl>) -> Self {
        Self { decls }
    }

    pub fn decls(&self) -> &[Decl] {
        &self.decls
    }
}
