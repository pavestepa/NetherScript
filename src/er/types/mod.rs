pub mod enum_type; pub use enum_type::EnumType;
pub mod prim_type; pub use prim_type::PrimType;
pub mod ref_type; pub use ref_type::RefType;

pub enum HasType {
  EnumType(EnumType),
  PrimType(PrimType),
  RefType(RefType)
}