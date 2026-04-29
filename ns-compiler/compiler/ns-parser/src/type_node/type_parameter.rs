use ns_ast::TypeParameter;
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_type_parameter(&mut self) -> TypeParameter {
        let ident = self.parse_ident();

        let mut implements = Vec::new();
        if matches!(
            self.current().kind,
            TokenKind::Keyword(Keyword::Implements)
        ) {
            self.parse(TokenKind::Keyword(Keyword::Implements));
            implements.push(self.parse_ident());
            while self.current().kind == TokenKind::Plus {
                self.parse(TokenKind::Plus);
                implements.push(self.parse_ident());
            }
        }

        let default_type = if self.current().kind == TokenKind::Assign {
            self.parse(TokenKind::Assign);
            Some(self.parse_type_node())
        } else {
            None
        };

        TypeParameter {
            ident,
            implements,
            default_type,
        }
    }

    pub fn parse_type_parameters_in_angle_brackets(&mut self) -> Vec<TypeParameter> {
        if self.current().kind != TokenKind::Less {
            return Vec::new();
        }

        self.parse(TokenKind::Less);

        if self.current().kind == TokenKind::Greater {
            self.parse(TokenKind::Greater);
            return Vec::new();
        }

        let mut out = Vec::new();
        loop {
            out.push(self.parse_type_parameter());

            match self.current().kind {
                TokenKind::Comma => {
                    self.parse(TokenKind::Comma);
                    if self.current().kind == TokenKind::Greater {
                        self.parse(TokenKind::Greater);
                        return out;
                    }
                }
                TokenKind::Greater => {
                    self.parse(TokenKind::Greater);
                    return out;
                }
                other => {
                    self.panic_at_current(format!(
                        "in type parameter list: expected ',' or '>', found {:?}",
                        other
                    ));
                }
            }
        }
    }
}
