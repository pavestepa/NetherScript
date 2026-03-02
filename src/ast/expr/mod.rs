mod binary_op;
mod binding_call;
mod function_call;
mod literal_call;
mod logical_op;
mod member_call;
mod unary_op;

pub use binary_op::{BinaryOp, BinaryOperator};
pub use binding_call::BindignCall;
pub use function_call::FunctionCall;
pub use literal_call::LiteralCall;
pub use logical_op::{LogicalOp, LogicalOperator};
pub use member_call::MemberCall;
pub use unary_op::UnaryOp;

#[derive(Debug, Clone)]
pub struct Expr {
    scoped: bool,
    expr_kind: Box<ExprKind>,
}
#[derive(Debug, Clone)]
pub enum ExprKind {
    BindignCall(BindignCall),
    FunctionCall(FunctionCall),
    LiteralCall(LiteralCall),
    MemberCall(MemberCall),
    BinaryOp(BinaryOp),
    LogicalOp(LogicalOp),
    UnaryOp(UnaryOp),
    Expr(Expr),
}
