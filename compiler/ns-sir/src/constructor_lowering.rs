use crate::decl::SirProgram;
use crate::decl::SirDecl;
use crate::diagnostics::{SirResult, sir_error};
use crate::ids::SirFnId;
use crate::stmt::SirStmt;
use crate::symbols::SirFunctionSymbol;

pub fn normalize_constructors(program: &mut SirProgram) -> SirResult<()> {
    let mut next_fn_id = program
        .functions
        .iter()
        .map(|f| f.id.0)
        .max()
        .unwrap_or(0)
        .saturating_add(1);

    let class_snapshots = program
        .classes
        .iter()
        .map(|c| (c.id, c.name.clone(), c.inherited_class, c.fields.clone()))
        .collect::<Vec<_>>();

    for (class_id, class_name, inherited_class, fields) in class_snapshots {
        let class_decl_ctor = class_constructor_decl(program, class_id);
        match class_decl_ctor {
            Some(existing_ctor) => {
                let Some(body) = function_body_mut(program, existing_ctor) else {
                    return Err(vec![sir_error(
                        "SIR2001",
                        format!("constructor fn {} body missing for class `{class_name}`", existing_ctor.0),
                    )]);
                };
                let mut normalized = constructor_prefix(inherited_class, &fields);
                normalized.extend(std::mem::take(body));
                *body = normalized;
            }
            None => {
                let ctor_id = SirFnId(next_fn_id);
                next_fn_id = next_fn_id.saturating_add(1);
                program.functions.push(SirFunctionSymbol {
                    id: ctor_id,
                    name: format!("{class_name}::constructor"),
                    params: Vec::new(),
                    ret: None,
                });
                program.decls.push(SirDecl::Function {
                    id: ctor_id,
                    body: constructor_prefix(inherited_class, &fields),
                });
                set_class_constructor_decl(program, class_id, ctor_id);
            }
        }
    }
    Ok(())
}

fn constructor_prefix(
    inherited_class: Option<crate::ids::SirClassId>,
    fields: &[crate::ids::SirFieldId],
) -> Vec<SirStmt> {
    let mut out = Vec::new();
    if let Some(parent) = inherited_class {
        out.push(SirStmt::InitParent { class_id: parent });
    }
    for field in fields {
        out.push(SirStmt::InitField { field_id: *field });
    }
    out
}

fn class_constructor_decl(program: &SirProgram, class_id: crate::ids::SirClassId) -> Option<SirFnId> {
    for decl in &program.decls {
        if let SirDecl::Class { id, constructor, .. } = decl {
            if *id == class_id {
                return *constructor;
            }
        }
    }
    None
}

fn set_class_constructor_decl(program: &mut SirProgram, class_id: crate::ids::SirClassId, ctor: SirFnId) {
    for decl in &mut program.decls {
        if let SirDecl::Class { id, constructor, .. } = decl {
            if *id == class_id {
                *constructor = Some(ctor);
                return;
            }
        }
    }
}

fn function_body_mut(program: &mut SirProgram, fn_id: SirFnId) -> Option<&mut Vec<SirStmt>> {
    for decl in &mut program.decls {
        if let SirDecl::Function { id, body } = decl {
            if *id == fn_id {
                return Some(body);
            }
        }
    }
    None
}
