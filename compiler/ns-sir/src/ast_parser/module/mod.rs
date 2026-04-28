use ns_ast::{Decl, Export, Module};

use crate::ast_parser::Lowerer;
use crate::decl::{SirDecl, SirImplTarget};
use crate::diagnostics::SirResult;
use crate::ids::{SirClassId, SirEnumId, SirTraitId};
use crate::symbols::{SirClassSymbol, SirExportLink, SirImportLink, SirLinkTarget, SirTraitSymbol};

pub(crate) fn lower_module(lowerer: &mut Lowerer, module: &Module) -> SirResult<()> {
    predeclare_classes(lowerer, module);
    predeclare_functions(lowerer, module);
    predeclare_module_values(lowerer, module);
    lowerer.push_scope();
    prebind_module_values(lowerer, module);
    for module_decl in module.decls() {
        lowerer.lower_decl(module_decl.decl())?;
    }
    lowerer.pop_scope();
    lower_module_links(lowerer, module);
    Ok(())
}

pub(crate) fn predeclare_functions(lowerer: &mut Lowerer, module: &Module) {
    for module_decl in module.decls() {
        if let Decl::Function(f) = module_decl.decl() {
            let fn_name = f.signature.ident.clone().into_simple().as_str().to_string();
            if lowerer.find_function_id(&fn_name).is_none() {
                lowerer.alloc_fn_symbol(fn_name, Vec::new(), None);
            }
        }
    }
}

pub(crate) fn predeclare_module_values(lowerer: &mut Lowerer, module: &Module) {
    for module_decl in module.decls() {
        if let Decl::Const(c) = module_decl.decl() {
            let name = c.binding.ident.clone().into_simple().as_str().to_string();
            if !lowerer.program.values.iter().any(|value| value.name == name) {
                let ty = lowerer.lower_type(&c.binding.type_ref);
                let _ = lowerer.alloc_value(&name, Some(ty));
            }
        }
    }
}

pub(crate) fn prebind_module_values(lowerer: &mut Lowerer, module: &Module) {
    for module_decl in module.decls() {
        if let Decl::Const(c) = module_decl.decl() {
            let name = c.binding.ident.clone().into_simple().as_str().to_string();
            if let Some(value_id) = lowerer
                .program
                .values
                .iter()
                .find(|value| value.name == name)
                .map(|value| value.id)
            {
                lowerer.declare_value(name, value_id);
            }
        }
    }
}

pub(crate) fn predeclare_classes(lowerer: &mut Lowerer, module: &Module) {
    for module_decl in module.decls() {
        if let Decl::Class(c) = module_decl.decl() {
            let class_name = c.ident.clone().into_simple().as_str().to_string();
            let class_id = SirClassId(lowerer.next_class_id);
            lowerer.next_class_id += 1;
            lowerer.class_by_name.insert(class_name.clone(), class_id);

            let trait_id = SirTraitId(lowerer.next_trait_id);
            lowerer.next_trait_id += 1;
            lowerer.trait_by_class_name.insert(class_name.clone(), trait_id);

            lowerer.program.classes.push(SirClassSymbol {
                id: class_id,
                name: class_name.clone(),
                fields: Vec::new(),
                methods: Vec::new(),
                inherited_class: None,
                implemented_traits: Vec::new(),
                delegated_trait_impls: Vec::new(),
            });
            lowerer.program.traits.push(SirTraitSymbol {
                id: trait_id,
                name: format!("{class_name}Trait"),
                methods: Vec::new(),
            });
        }
    }
}

pub(crate) fn lower_module_links(lowerer: &mut Lowerer, module: &Module) {
    for import in module.imports() {
        let local_name = import.ident.clone().into_simple().as_str().to_string();
        let from_module_path = import
            .from
            .iter()
            .map(|i| i.clone().into_simple().as_str().to_string())
            .collect::<Vec<_>>()
            .join(".");
        lowerer.program.imports.push(SirImportLink {
            target: resolve_link_target(lowerer, &local_name),
            local_name,
            from_module_path,
        });
    }

    for export in module.exports() {
        match export {
            Export::Ident(ident) => {
                let name = ident.clone().into_simple().as_str().to_string();
                lowerer.program.exports.push(SirExportLink {
                    target: resolve_link_target(lowerer, &name),
                    name,
                });
            }
            Export::Idents(idents) => {
                for ident in idents {
                    let name = ident.clone().into_simple().as_str().to_string();
                    lowerer.program.exports.push(SirExportLink {
                        target: resolve_link_target(lowerer, &name),
                        name,
                    });
                }
            }
        }
    }
}

pub(crate) fn resolve_link_target(lowerer: &Lowerer, name: &str) -> Option<SirLinkTarget> {
    if let Some(v) = lowerer
        .program
        .values
        .iter()
        .find(|v| v.name == name)
        .map(|v| SirLinkTarget::Value(v.id))
    {
        return Some(v);
    }
    if let Some(f) = lowerer
        .program
        .functions
        .iter()
        .find(|f| f.name == name || f.name.ends_with(&format!("::{name}")))
        .map(|f| SirLinkTarget::Function(f.id))
    {
        return Some(f);
    }
    if let Some(c) = lowerer
        .program
        .classes
        .iter()
        .find(|c| c.name == name)
        .map(|c| SirLinkTarget::Class(c.id))
    {
        return Some(c);
    }
    if let Some(e) = lowerer
        .program
        .enums
        .iter()
        .find(|e| e.name == name)
        .map(|e| SirLinkTarget::Enum(e.id))
    {
        return Some(e);
    }
    lowerer
        .program
        .traits
        .iter()
        .find(|t| t.name == name || t.name == format!("{name}Trait"))
        .map(|t| SirLinkTarget::Trait(t.id))
}

pub(crate) fn push_impl_for_enum(lowerer: &mut Lowerer, enum_id: SirEnumId, methods: Vec<crate::ids::SirFnId>) {
    lowerer.program.decls.push(SirDecl::Impl {
        target: SirImplTarget::Enum(enum_id),
        methods,
    });
}
