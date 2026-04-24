mod dynamic_type;
mod named_type;
mod type_parameter;

pub use {
    dynamic_type::DynamicType, named_type::NamedType, type_parameter::TypeParameter,
};

/// Types in parameter and return positions (`T`, `Map<K,V>`, `dynamic IFace`).
#[derive(Debug, Clone)]
pub enum TypeNode {
    /// Identifier with optional type arguments: `T`, `Map<K, V>`.
    Named(NamedType),
    /// `dynamic InterfaceName` — call interface methods with dynamic dispatch.
    Dynamic(DynamicType),
}
