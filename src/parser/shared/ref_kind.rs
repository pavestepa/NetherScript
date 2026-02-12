use crate::{
    ast::RefKind,
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn try_parse_ref_kind(&mut self) -> Result<RefKind, String> {
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                Keyword::Ref => {
                    self.parse(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Ref);
                }
                Keyword::Var => {
                    self.parse(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Var);
                }
                Keyword::Own => {
                    self.parse(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Own);
                }
                e => {
                    return Err(format!("expexted 'ref', 'var' or 'own', but found {:?}", e));
                }
            }
        } else {
            Err(format!(
                "expexted keyword 'ref', 'var' or 'own', but found {:?}",
                self.current().kind
            ))
        }
    }
}
