use crate::{
    ast::RefKind,
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn try_parse_ref_kind(&mut self) -> Result<RefKind, String> {
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                Keyword::Read => {
                    self.parse(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Change);
                }
                Keyword::Change => {
                    self.parse(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Change);
                }
                e => {
                    return Err(format!("expexted 'read' or 'change', but found {:?}", e));
                }
            }
        } else {
            Err(format!("is not keyword, found{:?}", self.current().kind))
        }
    }
}
