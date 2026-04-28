use ns_ast::TypeDecl;

use crate::ast_parser::Lowerer;
use crate::decl::SirDecl;
use crate::diagnostics::SirResult;

pub(crate) fn lower(lowerer: &mut Lowerer, t: &TypeDecl) -> SirResult<()> {
    let name = t.ident.clone().into_simple().as_str().to_string();
    let ty = lowerer.lower_type(&t.assign);
    lowerer.program.decls.push(SirDecl::TypeAlias { name, ty });
    Ok(())
}
