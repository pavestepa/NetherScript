use crate::decl::{SirDecl, SirImplTarget, SirProgram};
use crate::diagnostics::{SirResult, sir_error};
use crate::ids::{SirClassId, SirFnId, SirTraitId, SirValueId};
use crate::symbols::{SirFunctionSymbol, SirTraitImpl, SirTraitSymbol, SirValueSymbol};

/// Class policy lowering pass.
///
/// Policy: every class synthesizes field accessors (`get_<field>`, `set_<field>`) and
/// places them into the class self-trait method set. If this should become configurable,
/// move accessor synthesis into an optional dedicated pass.
pub fn lower_classes(program: &mut SirProgram) -> SirResult<()> {
    verify_no_inheritance_cycles(program)?;

    let mut next_trait_id = program
        .traits
        .iter()
        .map(|t| t.id.0)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    let mut next_fn_id = program
        .functions
        .iter()
        .map(|f| f.id.0)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    let mut next_value_id = program
        .values
        .iter()
        .map(|v| v.id.0)
        .max()
        .unwrap_or(0)
        .saturating_add(1);

    let class_snapshots = program
        .classes
        .iter()
        .map(|c| {
            (
                c.id,
                c.name.clone(),
                c.fields.clone(),
                c.methods.clone(),
                c.inherited_class,
            )
        })
        .collect::<Vec<_>>();

    for (class_id, class_name, fields, methods, inherited_class) in class_snapshots {
        let self_trait_id = ensure_class_trait(program, &class_name, &mut next_trait_id);

        let mut trait_methods = methods.clone();
        for field_id in &fields {
            let Some((field_name, field_ty)) = program
                .fields
                .iter()
                .find(|f| f.id == *field_id)
                .map(|f| (f.name.clone(), f.ty))
            else {
                return Err(vec![sir_error(
                    "SIR3006",
                    format!("class `{class_name}` references unknown field id {}", field_id.0),
                )]);
            };
            let getter_id = alloc_fn(
                program,
                &mut next_fn_id,
                format!("{class_name}::get_{field_name}"),
                Vec::new(),
                field_ty,
            );
            let setter_param_id = alloc_value(program, &mut next_value_id, "value".to_string(), field_ty);
            let setter_id = alloc_fn(
                program,
                &mut next_fn_id,
                format!("{class_name}::set_{field_name}"),
                vec![setter_param_id],
                None,
            );
            trait_methods.push(getter_id);
            trait_methods.push(setter_id);
        }
        set_trait_methods(program, self_trait_id, trait_methods.clone());

        let mut implemented_traits = vec![self_trait_id];
        let mut delegated_trait_impls = Vec::new();
        if let Some(parent_id) = inherited_class {
            let Some(parent_name) = class_name_of(program, parent_id).map(|v| v.to_string()) else {
                return Err(vec![sir_error(
                    "SIR3001",
                    format!("class `{class_name}` references unknown inherited class id {}", parent_id.0),
                )]);
            };
            let parent_trait_id = ensure_class_trait(program, &parent_name, &mut next_trait_id);
            implemented_traits.push(parent_trait_id);
            delegated_trait_impls.push(SirTraitImpl {
                trait_id: parent_trait_id,
                via_member: "inherited_class".to_string(),
            });
        }

        if let Some(class_symbol) = program.classes.iter_mut().find(|c| c.id == class_id) {
            class_symbol.implemented_traits = implemented_traits;
            class_symbol.delegated_trait_impls = delegated_trait_impls;
        }
        upsert_class_impl_decl(program, class_id, methods);
    }

    Ok(())
}

fn verify_no_inheritance_cycles(program: &SirProgram) -> SirResult<()> {
    for class in &program.classes {
        let mut seen = std::collections::HashSet::new();
        let mut current = class.inherited_class;
        while let Some(parent_id) = current {
            if !seen.insert(parent_id) {
                return Err(vec![sir_error(
                    "SIR3002",
                    format!("inheritance cycle detected at class `{}`", class.name),
                )]);
            }
            current = program
                .classes
                .iter()
                .find(|c| c.id == parent_id)
                .and_then(|c| c.inherited_class);
        }
    }
    Ok(())
}

fn ensure_class_trait(program: &mut SirProgram, class_name: &str, next_trait_id: &mut u32) -> SirTraitId {
    let target_name = format!("{class_name}Trait");
    if let Some(existing) = program.traits.iter().find(|t| t.name == target_name) {
        return existing.id;
    }
    let id = SirTraitId(*next_trait_id);
    *next_trait_id = next_trait_id.saturating_add(1);
    program.traits.push(SirTraitSymbol {
        id,
        name: target_name,
        methods: Vec::new(),
    });
    id
}

fn set_trait_methods(program: &mut SirProgram, trait_id: SirTraitId, methods: Vec<SirFnId>) {
    if let Some(tr) = program.traits.iter_mut().find(|t| t.id == trait_id) {
        tr.methods = methods;
    }
}

fn class_name_of(program: &SirProgram, class_id: SirClassId) -> Option<&str> {
    program
        .classes
        .iter()
        .find(|c| c.id == class_id)
        .map(|c| c.name.as_str())
}

fn alloc_fn(
    program: &mut SirProgram,
    next_fn_id: &mut u32,
    name: String,
    params: Vec<SirValueId>,
    ret: Option<crate::ids::SirTypeId>,
) -> SirFnId {
    let id = SirFnId(*next_fn_id);
    *next_fn_id = next_fn_id.saturating_add(1);
    program.functions.push(SirFunctionSymbol { id, name, params, ret });
    id
}

fn alloc_value(
    program: &mut SirProgram,
    next_value_id: &mut u32,
    name: String,
    ty: Option<crate::ids::SirTypeId>,
) -> SirValueId {
    let id = SirValueId(*next_value_id);
    *next_value_id = next_value_id.saturating_add(1);
    program.values.push(SirValueSymbol { id, name, ty });
    id
}

fn upsert_class_impl_decl(program: &mut SirProgram, class_id: SirClassId, methods: Vec<SirFnId>) {
    if let Some(decl) = program.decls.iter_mut().find(|d| {
        matches!(
            d,
            SirDecl::Impl {
                target: SirImplTarget::Class(id),
                ..
            } if *id == class_id
        )
    }) {
        if let SirDecl::Impl { methods: slot, .. } = decl {
            *slot = methods;
        }
        return;
    }
    program.decls.push(SirDecl::Impl {
        target: SirImplTarget::Class(class_id),
        methods,
    });
}
