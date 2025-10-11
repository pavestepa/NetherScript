mod prim_type; pub use prim_type::PrimType;
mod ref_type; pub use ref_type::RefType;

pub enum HasType {
  PrimType(PrimType),
  RefType(RefType)
}
