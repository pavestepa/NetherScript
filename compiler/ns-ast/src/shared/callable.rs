use crate::{Ident, This, TypeNode, TypeParameter, TypedBinding};

/// Callable signature: name, optional type parameters (`<T>`), receiver, parameters, return type.
#[derive(Debug)]
pub struct Callable {
    pub ident: Ident,
    /// Type parameters on the callable, e.g. `<T>` in `function identity<T>(…)`.
    pub type_parameters: Vec<TypeParameter>,
    /// `Static` or a Rust-style explicit receiver (`self` / `&self` / …).
    pub this: This,
    pub arguments: Vec<TypedBinding>,
    pub return_type: TypeNode,
}
