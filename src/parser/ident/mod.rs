use crate::{ast::Ident, lexer::TokenKind, parser::Parser};

impl Parser {
    pub fn parse_ident(&mut self) -> Result<Ident, String> {
        match self.token() {
            TokenKind::Ident(ident) => {
                self.next();
                Ok(Ident(ident))
            }
            e => Err(format!("can not use {:?} for identificate name", e)),
        }
    }
}
