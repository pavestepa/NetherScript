use crate::{type_node::TypeNode, Ident};

/// Generic parameter: `<T>`, `<T implements ToString>`, `<T implements A + B>`, optionally `= U`.
///
/// Several interfaces use `+` (like Rust trait `+`), not comma, so `<T, U>` stays unambiguous.
#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub ident: Ident,
    /// `T implements Foo + Bar` — interface bounds.
    pub implements: Vec<Ident>,
    pub default_type: Option<TypeNode>,
}
