use crate::ast::stmt::ArgStmt;
use crate::ast::{decl::Decl, decl::FnDecl, Typ};
use crate::lexer::Token;
use super::Parser;

impl Parser {
    pub fn parse_function_decl(&mut self, is_pub: bool) -> Result<Decl, String> {
        let name = match self.advance() {
            Some(Token::Ident(n)) => n,
            _ => return Err("Expected function name".into()),
        };

        self.expect(Token::LeftParen)?;
        let args = self.parse_parameter_list()?;
        self.expect(Token::RightParen)?;

        let returns = if self.check(&Token::Assign)
            && self.peek_ahead(1) == Some(&Token::GreaterThan)
        {
            self.advance();
            self.advance();
            self.parse_type()?
        } else {
            Typ::Void
        };

        self.expect(Token::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::RightBrace)?;

        Ok(Decl::FnDecl(FnDecl::new(
            is_pub, name, args, returns, body,
        )))
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<ArgStmt>, String> {
        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                // Parse parameter name
                let ident = match self.advance() {
                    Some(Token::Ident(name)) => name,
                    other => return Err(format!("Expected parameter name, found {:?}", other)),
                };

                // Parse optional type annotation
                let type_kind = if self.check(&Token::Assign) && self.peek_ahead(1) == Some(&Token::GreaterThan) {
                    // Skip "=>" for now, just parse type
                    self.advance(); // "="
                    self.advance(); // ">"
                    self.parse_type()?
                } else {
                    Typ::Void // Default type if not specified
                };

                params.push(ArgStmt::new(ident, type_kind));

                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        Ok(params)
    }
}
