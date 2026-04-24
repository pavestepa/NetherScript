use crate::{Ident, RefKind};

/// Run-time binding to a named interface; the concrete implementation is chosen when the value is used.
#[derive(Debug, Clone)]
pub struct DynamicType {
    /// Whether the binding is owned, immutably borrowed, or exclusively mutably borrowed.
    pub ref_kind: RefKind,
    pub interface: Ident,
}
