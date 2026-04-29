use crate::{Field, Ident, Method, TypeParameter};

/// Class declaration (`extends` / `implements` in a TypeScript-like surface).
///
/// Generic parameters on the class itself, e.g. `class Box<T> { … }`, live in [`ClassDecl::type_parameters`].
/// Fields use [`Field`] (optional type + optional initializer). Methods are [`Method`]; see
/// [`crate::Callable::this`] is either [`crate::This::Static`] or [`crate::This::Receiver`].
#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub ident: Ident,
    /// Type parameters from the class header, e.g. `<T>` or `<T implements ToString + Debug>`.
    pub type_parameters: Vec<TypeParameter>,
    /// Single supertype when inheritance is specified.
    pub extends: Option<Ident>,
    /// Interface names whose contracts this class must satisfy when the list is present.
    pub implements: Option<Vec<Ident>>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}
