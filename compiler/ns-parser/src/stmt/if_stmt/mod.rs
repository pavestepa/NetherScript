use ns_ast::{IfStmt, ast::Ast};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_if_stmt(&mut self) -> Ast<IfStmt> {
        let test = Box::from(self.parse_expr(0));
        let body = Box::from(self.parse_stmts_block());
        let alt = match self.current().kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Else => {
                    self.parse(TokenKind::Keyword(keyword));
                    Some(Box::from(self.parse_stmts_block()))
                }
                _ => None,
            },
            _ => None,
        };
        Ast::Parsed(IfStmt { test, body, alt })
    }
}
