mod binding_call;
mod function_call;
mod literal_call;
mod member_call;

pub use binding_call::BindignCall;
pub use function_call::FunctionCall;
pub use literal_call::LiteralCall;
pub use member_call::MemberCall;

#[derive(Debug, Clone)]
pub enum CallExpr {
    Function(FunctionCall),
    Literal(LiteralCall),
    Member(MemberCall),
    Binding(BindignCall),
    Error,
}
