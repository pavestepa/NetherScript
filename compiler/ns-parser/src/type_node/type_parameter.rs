use ns_ast::{TypeParameter, ast::Ast};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_type_parameter(&mut self) -> Ast<TypeParameter> {
        let ident = match self.parse_ident() {
            Ok(v) => v,
            Err(e) => return Ast::Error(e),
        };

        let mut implements = Vec::new();
        if matches!(
            self.current().kind,
            TokenKind::Keyword(Keyword::Implements)
        ) {
            self.parse(TokenKind::Keyword(Keyword::Implements));
            match self.parse_ident() {
                Ok(first) => implements.push(first),
                Err(e) => return Ast::Error(e),
            }
            while self.current().kind == TokenKind::Plus {
                self.parse(TokenKind::Plus);
                match self.parse_ident() {
                    Ok(id) => implements.push(id),
                    Err(e) => return Ast::Error(e),
                }
            }
        }

        let default_type = if self.current().kind == TokenKind::Assign {
            self.parse(TokenKind::Assign);
            match self.parse_type_node() {
                Ast::Parsed(t) => Some(t),
                Ast::Error(e) => return Ast::Error(e),
            }
        } else {
            None
        };

        Ast::Parsed(TypeParameter {
            ident,
            implements,
            default_type,
        })
    }

    /// Optional `<T, …>` after a name (class, function, …). Empty if the next token is not `<`.
    pub fn parse_type_parameters_in_angle_brackets(&mut self) -> Result<Vec<TypeParameter>, String> {
        if self.current().kind != TokenKind::Less {
            return Ok(Vec::new());
        }
        self.parse(TokenKind::Less);
        if self.current().kind == TokenKind::Greater {
            self.parse(TokenKind::Greater);
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        loop {
            match self.parse_type_parameter() {
                Ast::Parsed(p) => out.push(p),
                Ast::Error(e) => return Err(e),
            }
            match self.current().kind {
                TokenKind::Comma => {
                    self.parse(TokenKind::Comma);
                    if self.current().kind == TokenKind::Greater {
                        self.parse(TokenKind::Greater);
                        return Ok(out);
                    }
                }
                TokenKind::Greater => {
                    self.parse(TokenKind::Greater);
                    return Ok(out);
                }
                other => {
                    return Err(format!(
                        "in type parameter list: expected ',' or '>', found {:?}",
                        other
                    ));
                }
            }
        }
    }
}
