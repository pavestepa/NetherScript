mod binary;
mod fn_call;
mod logical;
mod member_call;
mod referencing;
mod unary;

pub use fn_call::FnCall;

use crate::{
    semantics::hir::exprs::{
        binary::Binary, logical::Logical, member_call::MemberCall, referencing::Referencing,
        unary::Unary,
    },
    Atom,
};

pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Logical(Logical),
    Referencing(Referencing),
    BindingCall(Atom),
    FnCall(FnCall),
    StringLiteral(Atom),
    NumberLiteral(Atom),
    BooleanLiteral(bool),
    MemberCall(MemberCall),
}
