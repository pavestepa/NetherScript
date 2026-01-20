use crate::{ast::TypeRef, lexer::TokenKind, parser::Parser};

impl Parser {
    pub fn parse_type_ref(&mut self) -> Result<TypeRef, String> {
        match self.token() {
            TokenKind::Ident(ident) => {
                self.next();
                Ok(TypeRef::Literal(ident))
            }
            e => Err(format!("can not use {:?} for identificate type", e)),
        }
    }
}
