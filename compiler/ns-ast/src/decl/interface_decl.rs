use crate::{Callable, Ident, StmtsBlock, TypeParameter};

/// Interface declaration; type parameters mirror `interface Box<T> { … }`.
#[derive(Debug)]
pub struct InterfaceDecl {
    pub ident: Ident,
    /// Header generics, e.g. `<T>` or `<T implements ToString + Debug>`.
    pub type_parameters: Vec<TypeParameter>,
    pub methods: Vec<(Callable, Option<StmtsBlock>)>,
}
