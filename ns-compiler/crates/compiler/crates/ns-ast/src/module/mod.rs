mod export;
mod import;

pub use export::Export;
pub use import::Import;

use std::collections::HashSet;

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

    pub fn into_merged_module(self) -> Module {
        let mut merged_decls = Vec::new();
        let mut merged_exports = Vec::new();
        let mut merged_imports = Vec::new();
        let mut merged_index = Vec::new();
        let entry_path = self.entry_path.clone();

        for package_module in self.modules {
            let is_entry = package_module.path == entry_path;
            let module = package_module.module;

            if is_entry {
                merged_decls.extend(module.decls);
                merged_exports.extend(module.exports);
                merged_imports.extend(module.imports);
                merged_index.extend(module.index);
                continue;
            }

            let mut exported_names = HashSet::new();
            for export in &module.exports {
                match export {
                    Export::Ident(ident) => {
                        exported_names.insert(ident_name(ident));
                    }
                    Export::Idents(idents) => {
                        for ident in idents {
                            exported_names.insert(ident_name(ident));
                        }
                    }
                }
            }

            for decl in module.decls {
                let is_exported_decl = decl.exported();
                let is_named_export = decl
                    .decl()
                    .name()
                    .map(|name| exported_names.contains(&name))
                    .unwrap_or(false);
                if is_exported_decl || is_named_export {
                    merged_decls.push(decl);
                }
            }
        }

        Module::new_full(merged_decls, merged_exports, merged_imports, merged_index)
    }
}

fn ident_name(ident: &Ident) -> String {
    ident.clone().into_simple().as_str().to_string()
}

impl Decl {
    fn name(&self) -> Option<String> {
        match self {
            Decl::Class(d) => Some(d.ident.clone().into_simple().as_str().to_string()),
            Decl::Const(d) => Some(d.binding.ident.clone().into_simple().as_str().to_string()),
            Decl::Enum(d) => Some(d.ident.clone().into_simple().as_str().to_string()),
            Decl::Function(d) => Some(d.signature.ident.clone().into_simple().as_str().to_string()),
            Decl::Interface(d) => Some(d.ident.clone().into_simple().as_str().to_string()),
            Decl::Type(d) => Some(d.ident.clone().into_simple().as_str().to_string()),
            Decl::TypeModifier(_) | Decl::Error(_) => None,
        }
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
