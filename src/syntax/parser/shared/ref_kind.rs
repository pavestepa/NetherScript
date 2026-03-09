use crate::syntax::{
    ast::RefKind,
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn try_parse_ref_kind(&mut self) -> Option<RefKind> {
        let mut parsed_ref_kind = None;
        let mut parsed_keyword = None;
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                Keyword::Read => {
                    parsed_keyword = Some(TokenKind::Keyword(keyword));
                    parsed_ref_kind = Some(RefKind::Read);
                }
                Keyword::Change => {
                    parsed_keyword = Some(TokenKind::Keyword(keyword));
                    parsed_ref_kind = Some(RefKind::Change);
                }
                e => return None,
            }
        } else {
            return None;
        }

        self.parse(parsed_keyword.unwrap());
        Some(parsed_ref_kind.unwrap())
    }
}
