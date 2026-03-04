mod binary_op;
mod binding_call;
mod function_call;
mod literal_call;
mod logical_op;
mod member_call;
mod referencing;
mod unary_op;

pub use binary_op::{BinaryOp, BinaryOperator};
pub use binding_call::BindignCall;
pub use function_call::FunctionCall;
pub use literal_call::LiteralCall;
pub use logical_op::{LogicalOp, LogicalOperator};
pub use member_call::MemberCall;
pub use referencing::Referencing;
pub use unary_op::UnaryOp;

#[derive(Debug, Clone)]
pub struct Expr {
    pub scoped: bool,
    pub expr_kind: Box<ExprKind>,
}
#[derive(Debug, Clone)]
pub enum ExprKind {
    //                             example             second token
    BindignCall(BindignCall),   // a
    FunctionCall(FunctionCall), // a()                 (
    LiteralCall(LiteralCall),   // true, "text", 4
    MemberCall(MemberCall),     // e.a                 .
    BinaryOp(BinaryOp),         // e + e               +
    LogicalOp(LogicalOp),       // e > e               >
    Referencing(Referencing),   // read e              e
    UnaryOp(UnaryOp),
}
