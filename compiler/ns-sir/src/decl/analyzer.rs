use crate::decl::SirDecl;

pub fn function_decl_count(decls: &[SirDecl]) -> usize {
    decls
        .iter()
        .filter(|d| matches!(d, SirDecl::Function { .. }))
        .count()
}
