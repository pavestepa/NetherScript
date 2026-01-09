use crate::{
    ast::{stmt::ReturnStmt, Expr},
    lexer::Token,
    parser::Parser,
};

impl Parser {
    pub fn parse_return_stmt(&mut self) -> Result<ReturnStmt, String> {
        match self.next().unwrap() {
            Token::Semicolon => Ok(ReturnStmt::new(None)),
            _ => Ok(ReturnStmt::new(Some(self.parse_expr(0)?))),
        }
    }
}
