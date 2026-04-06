mod function_type;
mod generic_type;
mod reference_type;
mod tuple_type;
mod type_parameter;

pub use function_type::FnType;
pub use generic_type::GenericType;
pub use reference_type::ReferenceType;
pub use tuple_type::TupleType;
pub use type_parameter::TypeParameter;

pub enum DataType {
    ReferenceType(ReferenceType),
    GenericType(GenericType),
    FnType(FnType),
    TupleType(TupleType),
}
