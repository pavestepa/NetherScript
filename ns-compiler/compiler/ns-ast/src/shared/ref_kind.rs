/// Whether a value is used by ownership or through a shared or exclusive borrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefKind {
    /// Owned value (no reference modifier).
    Own,
    /// Immutable shared reference: `&T`, expression `&expr`.
    Ref,
    /// Mutable exclusive reference: `&mut T`, expression `&mut expr`.
    Mut,
}
