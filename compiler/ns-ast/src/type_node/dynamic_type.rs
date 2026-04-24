use crate::Ident;

/// Dynamic dispatch against an interface: `dynamic ToString` (vtable / interface object).
#[derive(Debug, Clone)]
pub struct DynamicType {
    pub interface: Ident,
}
