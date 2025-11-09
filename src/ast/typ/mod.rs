use crate::Atom;

#[derive(Debug)]
pub enum Typ {
  Void,
  Boolean,
  Number,
  I8,
  I16,
  I32,
  I64,
  I128,
  U8,
  U16,
  U32,
  U64,
  U128,
  F16,
  F32,
  F64,
  F128,
  String,
  TypeLiteral(Atom)
}