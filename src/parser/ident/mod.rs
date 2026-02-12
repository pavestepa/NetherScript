use crate::{ast::Ident, lexer::TokenKind, parser::Parser};

impl Parser {
    pub fn parse_ident(&mut self) -> Result<Ident, String> {
        println!("[STARTED] parse Ident");
        match self.current().kind {
            TokenKind::Ident(ident) => {
                self.parse(TokenKind::Ident(ident));
                Ok(Ident(ident))
            }
            e => Err(format!("can not use {:?} for identificate name", e)),
        }
    }
}
