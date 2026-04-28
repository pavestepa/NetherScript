use crate::decl::{SirDecl, function_decl_count};

#[test]
fn counts_function_decls() {
    let decls = vec![SirDecl::Error, SirDecl::Function { id: crate::SirFnId(1), body: Vec::new() }];
    assert_eq!(function_decl_count(&decls), 1);
}
