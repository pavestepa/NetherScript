use ns_ast::Ident;
use ns_lexer::TokenKind;

use crate::Parser;


impl Parser {
    pub fn parse_ident(&mut self) -> Result<Ident, String> {
        println!("[STARTED] parse Ident");
        match self.current().kind {
            TokenKind::Ident(ident) => {
                self.parse(TokenKind::Ident(ident));
                Ok(Ident::new(ident))
            }
            e => Err(format!("can not use {:?} for identificate name", e)),
        }
    }
}
