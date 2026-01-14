mod function_call;
mod ident_call;
mod literal_call;
mod member_call;

pub use function_call::FunctionCall;
pub use ident_call::IdentCall;
pub use literal_call::LiteralCall;
pub use member_call::MemberCall;

#[derive(Debug, Clone)]
pub enum CallExpr {
    Function(FunctionCall),
    Literal(LiteralCall),
    Member(MemberCall),
    Ident(IdentCall),
    Error,
}
