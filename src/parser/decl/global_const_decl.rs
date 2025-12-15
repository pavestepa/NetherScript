use crate::{ast::decl::Decl, lexer::Token, parser::parser::Parser};

impl Parser {
  pub fn parse_global_const_decl(&mut self, is_pub: bool) -> Result<Decl, String> {
    let name = match self.advance() {
      Some(Token::Ident(n)) => n,
      _ => return Err("Expected function name".into())
    };

    self.expect(Token::Type(n))?; //TODO: DODELAY
    
  }
}