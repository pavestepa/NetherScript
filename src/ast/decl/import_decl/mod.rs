use crate::ast::Ident;

#[derive(Debug)]
pub struct ImportDecl {
    import: Ident,
    from: Vec<Ident>,
}
