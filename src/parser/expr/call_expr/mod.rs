use crate::parser::Parser;

mod binding_call;
mod function_call;
mod literal_call;
mod member_call;

impl Parser {
    pub fn parse_call_literal_or_binding(&mut self) -> Ast<Expr> {}
}
