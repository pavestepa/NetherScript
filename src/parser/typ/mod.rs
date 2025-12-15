use crate::ast::Typ;
use crate::lexer::Token;
use super::parser::Parser;

impl Parser {
    pub fn parse_type(&mut self) -> Result<Typ, String> {
        match self.advance() {
            Some(Token::Ident(name)) => match name.as_str().to_string().as_str() {
                "void" => Ok(Typ::Void),
                "number" => Ok(Typ::Number),
                "string" => Ok(Typ::String),
                _ => Ok(Typ::TypeLiteral(name)),
            },
            _ => Err("Expected type".into()),
        }
    }
}