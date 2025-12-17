use crate::{ast::Typ, lexer::Token};

use super::Parser;

impl Parser {
    pub fn parse_type(&self) -> Result<Typ, String> {
        let token = self.peek().unwrap();
        if self.peek().is_none() {
            return Err(format!(
                "expected Some(Token), but found {:?}",
                self.peek().unwrap()
            ));
        }
        if let Token::Ident(value) = *token {
            // TODO: optimize
            return match value.as_str().to_string().as_str() {
                "void" => Ok(Typ::Void),
                "boolean" => Ok(Typ::Boolean),
                "i8" => Ok(Typ::I8),
                "i16" => Ok(Typ::I16),
                "i32" => Ok(Typ::I32),
                "i64" => Ok(Typ::I64),
                "i128" => Ok(Typ::I128),
                "u8" => Ok(Typ::U8),
                "u16" => Ok(Typ::U16),
                "u32" => Ok(Typ::U32),
                "u64" => Ok(Typ::U64),
                "u128" => Ok(Typ::U128),
                "f32" => Ok(Typ::F32),
                "f64" => Ok(Typ::F64),
                "f128" => Ok(Typ::F128),
                "String" => Ok(Typ::String),
                "string" => Ok(Typ::String),
                _ => Ok(Typ::TypeLiteral(value)),
            };
        } else {
            return Err(format!(
                "expected Ident, but found {:?}",
                self.peek().unwrap()
            ));
        }
        Ok(Typ::Boolean)
    }
}
