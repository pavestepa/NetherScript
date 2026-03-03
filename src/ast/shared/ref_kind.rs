#[derive(Debug, Clone)]
pub enum RefKind {
    Read,
    Change,
    /* RawPtr, */
}
