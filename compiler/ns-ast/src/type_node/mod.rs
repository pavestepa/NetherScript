mod dynamic_type;
mod named_type;
mod type_parameter;

pub use {
    dynamic_type::DynamicType, named_type::NamedType, type_parameter::TypeParameter,
};

/// Types allowed wherever a type annotation is required, including nested positions inside other types.
#[derive(Debug, Clone)]
pub enum TypeNode {
    /// Named type with an optional ordered list of type arguments applied to that name.
    Named(NamedType),
    /// Run-time dispatch through a named interface, with an owned or borrowed passing mode.
    Dynamic(DynamicType),
}
