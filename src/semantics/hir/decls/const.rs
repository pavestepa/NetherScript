use crate::{
    semantics::hir::{data_type::DataType, exprs::Expr},
    Atom,
};

pub struct ConstDecl {
    pub id: Atom,
    pub data_type: DataType,
    pub val: Expr,
}
