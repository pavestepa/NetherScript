use crate::{ast::decl::{ClassDecl, Decl}, lexer::{Keyword, Token}, parser::parser::Parser};

impl Parser {
  pub fn parse_class_decl(&mut self, is_pub: bool) -> Result<Decl, String> {
    // Parse class name
    let ident = match self.advance() {
        Some(Token::Ident(name)) => name,
        other => return Err(format!("Expected class name, found {:?}", other)),
    };

    // Optional extends clause
    let extends = if self.check_keyword(Keyword::Extends) {
        self.advance();
        match self.advance() {
            Some(Token::Ident(name)) => Some(name),
            other => return Err(format!("Expected parent class name, found {:?}", other)),
        }
    } else {
        None
    };

    // Optional implements clause
    let implements = if self.check_keyword(Keyword::Implements) {
        self.advance();
        let mut interfaces = Vec::new();
        loop {
            match self.advance() {
                Some(Token::Ident(name)) => interfaces.push(name),
                other => return Err(format!("Expected interface name, found {:?}", other)),
            }
            if !self.check(&Token::Comma) {
                break;
            }
            self.advance(); // consume comma
        }
        Some(interfaces)
    } else {
        None
    };

    // Parse class body
    self.expect(Token::LeftBrace)?;
    let mut fields = Vec::new();
    
    // Parse class members (simplified - just collect field names)
    while !self.check(&Token::RightBrace) && !self.is_at_end() {
        if let Some(Token::Ident(field_name)) = self.peek() {
            fields.push(field_name.clone());
        }
        self.advance();
    }
    self.expect(Token::RightBrace)?;

    Ok(Decl::ClassDecl(ClassDecl::new(is_pub, ident, extends, implements, fields)))
}
}