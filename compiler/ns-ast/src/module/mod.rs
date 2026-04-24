mod export;
mod import;

pub use export::Export;
pub use import::Import;

use crate::{Ident, decl::Decl};

#[derive(Debug)]
pub struct ModuleDecl {
    decl: Decl,
    exported: bool,
}

impl ModuleDecl {
    pub fn new(decl: Decl, exported: bool) -> Self {
        Self { decl, exported }
    }

    pub fn decl(&self) -> &Decl {
        &self.decl
    }

    pub fn exported(&self) -> bool {
        self.exported
    }
}

#[derive(Debug)]
pub struct Module {
    decls: Vec<ModuleDecl>,
    exports: Vec<Export>,
    imports: Vec<Import>,
    index: Vec<Ident>,
}

impl Module {
    pub fn new(decls: Vec<ModuleDecl>) -> Self {
        Self {
            decls,
            exports: Vec::new(),
            imports: Vec::new(),
            index: Vec::new(),
        }
    }

    pub fn new_full(
        decls: Vec<ModuleDecl>,
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

    pub fn decls(&self) -> &[ModuleDecl] {
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
