use ns_ast::Module;

use crate::class_lowering::lower_classes;
use crate::constructor_lowering::normalize_constructors;
use crate::decl::SirProgram;
use crate::diagnostics::SirResult;
use crate::type_fill::fill_expr_types;
use crate::verify::{verify, verify_typed_sir};

pub fn lower_ast_module(module: &Module) -> SirResult<SirProgram> {
    let mut sir = crate::ast_parser::lower_ast_module(module)?;
    lower_classes(&mut sir)?;
    normalize_constructors(&mut sir)?;
    fill_expr_types(&mut sir);
    verify(&sir)?;
    verify_typed_sir(&sir)?;
    Ok(sir)
}

#[cfg(test)]
mod tests;
