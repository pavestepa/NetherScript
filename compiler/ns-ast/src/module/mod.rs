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

#[derive(Debug)]
pub struct PackageModule {
    path: String,
    module: Module,
}

impl PackageModule {
    pub fn new(path: String, module: Module) -> Self {
        Self { path, module }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn module(&self) -> &Module {
        &self.module
    }
}

#[derive(Debug)]
pub struct Package {
    entry_path: String,
    modules: Vec<PackageModule>,
}

impl Package {
    pub fn new(entry_path: String, modules: Vec<PackageModule>) -> Self {
        Self { entry_path, modules }
    }

    pub fn entry_path(&self) -> &str {
        &self.entry_path
    }

    pub fn modules(&self) -> &[PackageModule] {
        &self.modules
    }

    pub fn entry_module(&self) -> Option<&Module> {
        self.modules
            .iter()
            .find(|m| m.path() == self.entry_path)
            .map(PackageModule::module)
    }

    pub fn into_entry_module(self) -> Option<Module> {
        let entry = self.entry_path;
        self.modules
            .into_iter()
            .find(|m| m.path == entry)
            .map(|m| m.module)
    }
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
