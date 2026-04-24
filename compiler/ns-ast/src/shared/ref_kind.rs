/// How a value is borrowed or passed by reference (Rust-style `&` / `&mut`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefKind {
    /// Owned value (no reference modifier).
    Own,
    /// Immutable shared reference: `&T`, expression `&expr`.
    Ref,
    /// Mutable exclusive reference: `&mut T`, expression `&mut expr`.
    Mut,
}