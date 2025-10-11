pub enum RefType {
  Ref(String),
  RefMut(String),

  Rc(String),
  RefCell(String),
  RcRefCell(String),

  String,

  StringRc,
  StringRefCell,
  StringRcRefCell,

  Vec,

  VecRc,
  VecRefCell,
  VecRcRefCell,
}