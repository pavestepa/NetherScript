mod expect;

use crate::{
    ast::ConstDecl,
    atom,
    lexer::TokenKind,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_const_decl(&mut self) -> ParseResult<ConstDecl> {
        self.expect_let_or_const
    }
}
