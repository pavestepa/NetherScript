#[derive(Debug, Clone)]
pub enum RefKind {
    Own,
    Ref,
    Mut,
    /* Ptr */
    /* MutPtr, */
}
