use crate::{EnumMember, Ident, TypeParameter};

/// Enum declaration; type parameters mirror `enum Result<T, E> { … }`.
#[derive(Debug)]
pub struct EnumDecl {
    pub ident: Ident,
    /// Header generics, e.g. `<T>` or `<T implements ToString + Debug>`.
    pub type_parameters: Vec<TypeParameter>,
    pub members: Vec<EnumMember>,
}
