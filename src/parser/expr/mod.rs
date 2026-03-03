mod binary_op;
mod binding_call;
mod function_call;
mod literal_call;
mod logical_op;
mod member_call;
mod referencing;
mod unary_op;

use crate::{
    ast::{ast::Ast, Expr},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_expr(&mut self) -> Ast<Expr> {
        let mut scoped = false;
        let mut scope_closed = true;

        if let TokenKind::LeftParen = self.current().kind {
            scoped = true;
            scope_closed = false;
            self.parse(TokenKind::LeftParen);
        }
        // parse code here

        // TODO: expr parse

        Ast::Error("".to_string())
    }
}
