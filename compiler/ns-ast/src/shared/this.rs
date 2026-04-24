use crate::{RefKind, TypeNode};

/// Rust-style receiver: passing mode (`self`, `&self`, `&mut self`, …) and optional type annotation.
#[derive(Debug, Clone)]
pub struct ThisReceiver {
    /// `Own` — by value; `Ref` / `Mut` — shared or exclusive borrow.
    pub ref_kind: RefKind,
    /// Optional annotated receiver type; may be omitted when inferred from the class / impl.
    pub type_annotation: Option<TypeNode>,
}

/// Strict receiver discipline: no implicit instance `this` like TypeScript.
#[derive(Debug, Clone)]
pub enum This {
    /// No receiver — static item or module-level function.
    Static,
    /// Callable that takes a receiver as a real parameter (`self` / `&self` / `&mut self`, …).
    Receiver(ThisReceiver),
}
