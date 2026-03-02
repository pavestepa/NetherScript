use crate::{
    ast::{ast::Ast, IfStmt},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_if_stmt(&mut self) -> Ast<IfStmt> {
        let test = self.parse_expr();
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
        if let Ast::Error(e) = test {
            return Ast::Error(e);
        }
        Ast::Parsed(IfStmt {
            test: Box::new(test.unwrap()),
            body,
            alt,
        })
    }
}
