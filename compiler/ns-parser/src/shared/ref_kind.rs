use ns_ast::RefKind;
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn try_parse_ref_kind(&mut self) -> Option<RefKind> {
        let mut parsed_ref_kind = None;
        let mut parsed_keyword = None;
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                Keyword::Own => {
                    parsed_keyword = Some(TokenKind::Keyword(keyword));
                    parsed_ref_kind = Some(RefKind::Own);
                }
                Keyword::Ref => {
                    parsed_keyword = Some(TokenKind::Keyword(keyword));
                    parsed_ref_kind = Some(RefKind::Ref);
                }
                Keyword::Mut => {
                    parsed_keyword = Some(TokenKind::Keyword(keyword));
                    parsed_ref_kind = Some(RefKind::Mut);
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
