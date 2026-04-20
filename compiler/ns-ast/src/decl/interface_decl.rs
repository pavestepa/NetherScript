use crate::{Callable, Ident, StmtsBlock};

#[derive(Debug)]
pub struct InterfaceDecl {
    pub ident: Ident,
    pub methods: Vec<(Callable, Option<StmtsBlock>)>,
}
