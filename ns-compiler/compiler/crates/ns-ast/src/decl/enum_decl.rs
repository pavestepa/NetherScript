use crate::{EnumMember, Ident, Method, TypeParameter};

/// Enum declaration; type parameters mirror `enum Result<T, E> { … }`.
#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub ident: Ident,
    /// Header generics, e.g. `<T>` or `<T implements ToString + Debug>`.
    pub type_parameters: Vec<TypeParameter>,
    /// Interface names this enum implements.
    pub implements: Option<Vec<Ident>>,
    pub members: Vec<EnumMember>,
    pub methods: Vec<Method>,
}
