mod export;
mod import;

pub use export::Export;
pub use import::Import;

use crate::{Ident, decl::Decl};

#[derive(Debug)]
pub struct Module {
    decls: Vec<Decl>,
    exports: Vec<Export>,
    imports: Vec<Import>,
    index: Vec<Ident>,
}

impl Module {
    pub fn new(decls: Vec<Decl>) -> Self {
        Self {
            decls,
            exports: Vec::new(),
            imports: Vec::new(),
            index: Vec::new(),
        }
    }

    pub fn new_full(
        decls: Vec<Decl>,
        exports: Vec<Export>,
        imports: Vec<Import>,
        index: Vec<Ident>,
    ) -> Self {
        Self {
            decls,
            exports,
            imports,
            index,
        }
    }

    pub fn decls(&self) -> &[Decl] {
        &self.decls
    }

    pub fn exports(&self) -> &[Export] {
        &self.exports
    }

    pub fn imports(&self) -> &[Import] {
        &self.imports
    }

    pub fn index(&self) -> &[Ident] {
        &self.index
    }
}
