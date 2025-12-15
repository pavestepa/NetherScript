use crate::ast::{Module, decl::Decl};
use crate::lexer::{Token, Keyword};
use super::parser::Parser;

mod fn_decl;
mod class_decl;
mod global_const_decl;
mod type_decl;

impl Parser {
    pub fn parse_module(&mut self) -> Result<Module, String> {
        let mut decls = Vec::new();

        while !self.is_at_end() {
            match self.parse_decl() {
                Ok(d) => decls.push(d),
                Err(e) => {
                    eprintln!("Parse error: {e}");
                    self.synchronize();
                }
            }
        }

        Ok(Module::new(decls))
    }

    pub fn parse_decl(&mut self) -> Result<Decl, String> {
        let is_pub = if self.check_keyword(Keyword::Public) {
            self.advance();
            true
        } else {
            false
        };

        match self.peek() {
            Some(Token::Keyword(Keyword::Function)) => {
                self.advance();
                self.parse_fn_decl(is_pub)
            }
            Some(Token::Keyword(Keyword::Class)) => {
                self.advance();
                self.parse_class_decl(is_pub)
            }
            Some(Token::Keyword(Keyword::Const)) => {
                self.advance();
                self.parse_global_const()
            }
            _ => Err("Unexpected token at module level".into()),
        }
    }
}