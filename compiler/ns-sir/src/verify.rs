use crate::decl::{SirDecl, SirProgram};
use crate::diagnostics::{SirResult, sir_error};
use crate::expr::SirExpr;
use crate::ids::{SirClassId, SirEnumId, SirFnId, SirTraitId, SirValueId};
use crate::stmt::SirStmt;
use ns_diagnostics::Diagnostic;

pub fn verify(program: &SirProgram) -> SirResult<()> {
    for symbol in &program.values {
        if symbol.id.0 as usize >= program.values.len() {
            return Err(vec![sir_error(
                "SIR0001",
                format!("invalid value symbol id {}", symbol.id.0),
            )]);
        }
    }

    // Basic uniqueness check on value ids to catch malformed lowerers early.
    let mut seen = std::collections::HashSet::<SirValueId>::new();
    for symbol in &program.values {
        if !seen.insert(symbol.id) {
            return Err(vec![sir_error(
                "SIR0002",
                format!("duplicate value id {}", symbol.id.0),
            )]);
        }
    }

    for class in &program.classes {
        for trait_id in &class.implemented_traits {
            if !has_trait(program, *trait_id) {
                return Err(vec![sir_error(
                    "SIR0003",
                    format!("class `{}` references unknown trait id {}", class.name, trait_id.0),
                )]);
            }
        }
        for delegated in &class.delegated_trait_impls {
            if delegated.via_member.is_empty() {
                return Err(vec![sir_error(
                    "SIR0004",
                    format!("class `{}` has empty trait delegation member path", class.name),
                )]);
            }
            if !class.implemented_traits.contains(&delegated.trait_id) {
                return Err(vec![sir_error(
                    "SIR0013",
                    format!(
                        "class `{}` delegates trait {} that is not listed in implemented_traits",
                        class.name, delegated.trait_id.0
                    ),
                )]);
            }
        }
    }

    verify_impl_contracts(program)?;

    for decl in &program.decls {
        if let crate::decl::SirDecl::Class {
            id,
            constructor,
            fields,
            ..
        } = decl
        {
            let Some(ctor_id) = constructor else {
                return Err(vec![sir_error(
                    "SIR0005",
                    format!("class {} has no normalized constructor", id.0),
                )]);
            };
            verify_constructor_order(*id, *ctor_id, fields, program)?;
        }
    }

    for import in &program.imports {
        if import.target.is_none() {
            return Err(vec![sir_error(
                "SIR0010",
                format!(
                    "unresolved import link `{}` from `{}`",
                    import.local_name, import.from_module_path
                ),
            )]);
        }
    }

    for export in &program.exports {
        if export.target.is_none() {
            return Err(vec![sir_error(
                "SIR0011",
                format!("unresolved export link `{}`", export.name),
            )]);
        }
    }

    verify_referential_integrity(program)?;
    Ok(())
}

pub fn verify_typed_sir(program: &SirProgram) -> SirResult<()> {
    verify(program)?;
    let mut errors = Vec::new();
    for decl in &program.decls {
        match decl {
            SirDecl::Const { expr, .. } => verify_expr_has_type(expr, "const", &mut errors),
            SirDecl::Function { id, body } => {
                let fn_name = program
                    .functions
                    .iter()
                    .find(|f| f.id == *id)
                    .map(|f| f.name.as_str())
                    .unwrap_or("<unknown>");
                for stmt in body {
                    verify_stmt_expr_types(stmt, fn_name, &mut errors);
                }
            }
            _ => {}
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn has_trait(program: &SirProgram, id: SirTraitId) -> bool {
    program.traits.iter().any(|t| t.id == id)
}

fn verify_constructor_order(
    class_id: crate::ids::SirClassId,
    ctor_id: SirFnId,
    fields: &[crate::ids::SirFieldId],
    program: &SirProgram,
) -> SirResult<()> {
    let mut body_opt = None;
    for decl in &program.decls {
        if let crate::decl::SirDecl::Function { id, body } = decl {
            if *id == ctor_id {
                body_opt = Some(body);
                break;
            }
        }
    }
    let Some(body) = body_opt else {
        return Err(vec![sir_error(
            "SIR0006",
            format!("constructor function {} missing body", ctor_id.0),
        )]);
    };

    // Validate deterministic prefix: optional InitParent then all InitField entries.
    let mut idx = 0usize;
    if matches!(body.get(0), Some(SirStmt::InitParent { .. })) {
        idx += 1;
    }
    for field in fields {
        match body.get(idx) {
            Some(SirStmt::InitField { field_id }) if field_id == field => {
                idx += 1;
            }
            _ => {
                return Err(vec![sir_error(
                    "SIR0007",
                    format!(
                        "constructor order violation in class {}: missing InitField for {} at prefix",
                        class_id.0, field.0
                    ),
                )]);
            }
        }
    }
    Ok(())
}

fn verify_stmt_expr_types(stmt: &SirStmt, scope: &str, errors: &mut Vec<Diagnostic>) {
    match stmt {
        SirStmt::Expr(expr) => verify_expr_has_type(expr, scope, errors),
        SirStmt::Let { init, .. } => {
            if let Some(expr) = init {
                verify_expr_has_type(expr, scope, errors);
            }
        }
        SirStmt::Assign { target, value } => {
            verify_expr_has_type(target, scope, errors);
            verify_expr_has_type(value, scope, errors);
        }
        SirStmt::Return(ret) => {
            if let Some(expr) = ret {
                verify_expr_has_type(expr, scope, errors);
            }
        }
        SirStmt::If {
            test,
            then_body,
            else_body,
        } => {
            verify_expr_has_type(test, scope, errors);
            for nested in then_body {
                verify_stmt_expr_types(nested, scope, errors);
            }
            for nested in else_body {
                verify_stmt_expr_types(nested, scope, errors);
            }
        }
        SirStmt::Loop { body } => {
            for nested in body {
                verify_stmt_expr_types(nested, scope, errors);
            }
        }
        SirStmt::Error | SirStmt::InitParent { .. } | SirStmt::InitField { .. } | SirStmt::Break => {}
    }
}

fn verify_impl_contracts(program: &SirProgram) -> SirResult<()> {
    let mut class_impl_targets = std::collections::HashSet::<SirClassId>::new();
    let mut enum_impl_targets = std::collections::HashSet::<SirEnumId>::new();
    for decl in &program.decls {
        let SirDecl::Impl { target, methods } = decl else {
            continue;
        };
        for method_id in methods {
            if !program.functions.iter().any(|f| f.id == *method_id) {
                return Err(vec![sir_error(
                    "SIR0014",
                    format!("impl references unknown function id {}", method_id.0),
                )]);
            }
        }
        match target {
            crate::decl::SirImplTarget::Class(class_id) => {
                if !class_impl_targets.insert(*class_id) {
                    return Err(vec![sir_error(
                        "SIR0015",
                        format!("duplicate class impl target {}", class_id.0),
                    )]);
                }
                let Some(class_symbol) = program.classes.iter().find(|c| c.id == *class_id) else {
                    return Err(vec![sir_error(
                        "SIR0016",
                        format!("class impl points to unknown class id {}", class_id.0),
                    )]);
                };
                if class_symbol.methods != *methods {
                    return Err(vec![sir_error(
                        "SIR0017",
                        format!("class impl methods mismatch for class `{}`", class_symbol.name),
                    )]);
                }
            }
            crate::decl::SirImplTarget::Enum(enum_id) => {
                if !enum_impl_targets.insert(*enum_id) {
                    return Err(vec![sir_error(
                        "SIR0018",
                        format!("duplicate enum impl target {}", enum_id.0),
                    )]);
                }
                if !program.enums.iter().any(|e| e.id == *enum_id) {
                    return Err(vec![sir_error(
                        "SIR0019",
                        format!("enum impl points to unknown enum id {}", enum_id.0),
                    )]);
                }
            }
        }
    }
    Ok(())
}

fn verify_referential_integrity(program: &SirProgram) -> SirResult<()> {
    for decl in &program.decls {
        match decl {
            SirDecl::Const { expr, .. } => verify_expr_refs(expr, program)?,
            SirDecl::Function { body, .. } => {
                for stmt in body {
                    verify_stmt_refs(stmt, program)?;
                }
            }
            SirDecl::Enum { id } => {
                if !program.enums.iter().any(|e| e.id == *id) {
                    return Err(vec![sir_error(
                        "SIR0020",
                        format!("enum decl references unknown enum id {}", id.0),
                    )]);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_stmt_refs(stmt: &SirStmt, program: &SirProgram) -> SirResult<()> {
    match stmt {
        SirStmt::Expr(expr) => verify_expr_refs(expr, program),
        SirStmt::Let { init, .. } => {
            if let Some(expr) = init {
                verify_expr_refs(expr, program)?;
            }
            Ok(())
        }
        SirStmt::Assign { target, value } => {
            verify_expr_refs(target, program)?;
            verify_expr_refs(value, program)
        }
        SirStmt::Return(expr) => {
            if let Some(expr) = expr {
                verify_expr_refs(expr, program)?;
            }
            Ok(())
        }
        SirStmt::If {
            test,
            then_body,
            else_body,
        } => {
            verify_expr_refs(test, program)?;
            for nested in then_body {
                verify_stmt_refs(nested, program)?;
            }
            for nested in else_body {
                verify_stmt_refs(nested, program)?;
            }
            Ok(())
        }
        SirStmt::Loop { body } => {
            for nested in body {
                verify_stmt_refs(nested, program)?;
            }
            Ok(())
        }
        SirStmt::Error | SirStmt::InitParent { .. } | SirStmt::InitField { .. } | SirStmt::Break => Ok(()),
    }
}

fn verify_expr_refs(expr: &SirExpr, program: &SirProgram) -> SirResult<()> {
    match expr {
        SirExpr::TypeRef { type_id, .. } => {
            if type_id.0 as usize >= program.types.len() {
                return Err(vec![sir_error(
                    "SIR0021",
                    format!("TypeRef points to unknown type id {}", type_id.0),
                )]);
            }
        }
        SirExpr::Call { callee, args, .. } => {
            if let Some(fn_id) = callee {
                if !program.functions.iter().any(|f| f.id == *fn_id) {
                    return Err(vec![sir_error(
                        "SIR0022",
                        format!("Call points to unknown function id {}", fn_id.0),
                    )]);
                }
            }
            for arg in args {
                verify_expr_refs(arg, program)?;
            }
        }
        SirExpr::IntrinsicPrintln { args, .. } => {
            for arg in args {
                verify_expr_refs(arg, program)?;
            }
        }
        SirExpr::TemplateString { parts, .. } => {
            for part in parts {
                if let crate::expr::SirTemplatePart::Expr(expr) = part {
                    verify_expr_refs(expr, program)?;
                }
            }
        }
        SirExpr::Unary { value, .. } => verify_expr_refs(value, program)?,
        SirExpr::Binary { left, right, .. } => {
            verify_expr_refs(left, program)?;
            verify_expr_refs(right, program)?;
        }
        SirExpr::Member { object, .. } => verify_expr_refs(object, program)?,
        SirExpr::StructLiteral { fields, .. } => {
            for (_, value) in fields {
                verify_expr_refs(value, program)?;
            }
        }
        SirExpr::New { class_name, args, .. } => {
            if !program.classes.iter().any(|c| c.name == *class_name) {
                return Err(vec![sir_error(
                    "SIR0023",
                    format!("New references unknown class `{class_name}`"),
                )]);
            }
            for arg in args {
                verify_expr_refs(arg, program)?;
            }
        }
        SirExpr::Error { .. } | SirExpr::Literal { .. } | SirExpr::ValueRef { .. } => {}
    }
    Ok(())
}

fn verify_expr_has_type(
    expr: &SirExpr,
    scope: &str,
    errors: &mut Vec<Diagnostic>,
) {
    match expr {
        SirExpr::Error { .. } => {}
        SirExpr::Literal { ty, .. }
        | SirExpr::ValueRef { ty, .. }
        | SirExpr::TypeRef { ty, .. }
        | SirExpr::Call { ty, .. }
        | SirExpr::IntrinsicPrintln { ty, .. }
        | SirExpr::TemplateString { ty, .. }
        | SirExpr::Unary { ty, .. }
        | SirExpr::Binary { ty, .. }
        | SirExpr::Member { ty, .. }
        | SirExpr::StructLiteral { ty, .. }
        | SirExpr::New { ty, .. } => {
            if ty.is_none() {
                errors.push(sir_error(
                    "SIR0012",
                    format!("typed-SIR contract violation in `{scope}`: expression has no type"),
                ));
            }
        }
    }

    match expr {
        SirExpr::Call { args, .. } => {
            for arg in args {
                verify_expr_has_type(arg, scope, errors);
            }
        }
        SirExpr::IntrinsicPrintln { args, .. } => {
            for arg in args {
                verify_expr_has_type(arg, scope, errors);
            }
        }
        SirExpr::TemplateString { parts, .. } => {
            for part in parts {
                if let crate::expr::SirTemplatePart::Expr(expr) = part {
                    verify_expr_has_type(expr, scope, errors);
                }
            }
        }
        SirExpr::Unary { value, .. } => verify_expr_has_type(value, scope, errors),
        SirExpr::Binary { left, right, .. } => {
            verify_expr_has_type(left, scope, errors);
            verify_expr_has_type(right, scope, errors);
        }
        SirExpr::Member { object, .. } => verify_expr_has_type(object, scope, errors),
        SirExpr::StructLiteral { fields, .. } => {
            for (_, value) in fields {
                verify_expr_has_type(value, scope, errors);
            }
        }
        SirExpr::New { args, .. } => {
            for arg in args {
                verify_expr_has_type(arg, scope, errors);
            }
        }
        SirExpr::Error { .. } | SirExpr::Literal { .. } | SirExpr::ValueRef { .. } | SirExpr::TypeRef { .. } => {}
    }
}
