use ns_ast::{ClassDecl, Ident, Method, TypedBinding, ast::Ast};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_class_decl(&mut self) -> Ast<ClassDecl> {
        let ident = match self.parse_ident() {
            Ok(v) => v,
            Err(e) => {
                return Ast::Error(format!("expected name for class: {:?}", e));
            }
        };

        let type_parameters = match self.parse_type_parameters_in_angle_brackets() {
            Ok(v) => v,
            Err(e) => return Ast::Error(e),
        };

        let mut extends: Option<Ident> = None;
        let mut implements: Option<Vec<Ident>> = None;

        if let TokenKind::Keyword(keyword) = self.current().kind {
            if let Keyword::Extends = keyword {
                self.parse(TokenKind::Keyword(keyword));
                extends = match self.parse_ident() {
                    Ok(v) => Some(v),
                    Err(e) => {
                        return Ast::Error(format!(
                            "expected name for class for inheritance by 'extends': {:?}",
                            e
                        ));
                    }
                };
            }
            if let Keyword::Implements = keyword {
                self.parse(TokenKind::Keyword(keyword));
                implements = match self.parse_implements() {
                    Ok(v) => Some(v),
                    Err(e) => {
                        return Ast::Error(format!(
                            "expected names for implementing traits of class: {:?}",
                            e
                        ));
                    }
                }
            }
        }

        self.parse(TokenKind::LeftBrace);

        let (fields, methods) = self.parse_fields_and_methods();

        Ast::Parsed(ClassDecl {
            ident,
            type_parameters,
            extends,
            implements,
            fields,
            methods,
        })
    }

    fn parse_implements(&mut self) -> Result<Vec<Ident>, String> {
        let mut implements = vec![];
        while let TokenKind::Ident(ident) = self.current().kind {
            self.parse(TokenKind::Ident(ident));
            implements.push(Ident::new(ident));

            match self.current().kind {
                TokenKind::Comma => {
                    self.parse(TokenKind::Comma);
                }
                TokenKind::LeftBrace => {}
                e => {
                    return Err(format!("expected ',' or '{{' after trait name: {:?}", e));
                }
            }
        }

        Ok(implements)
    }

    fn parse_fields_and_methods(&mut self) -> (Vec<Ast<TypedBinding>>, Vec<Ast<Method>>) {
        let mut fields = vec![];
        let mut methods = vec![];

        while let TokenKind::Ident(_) = self.current().kind {
            match self.peek(1).kind {
                TokenKind::Colon => {
                    fields.push(self.parse_typed_binding());
                }
                TokenKind::LeftParen => {
                    methods.push(self.parse_method());
                }
                _ => b,
            }
        }

        (fields, methods)
    }
}
