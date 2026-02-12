use crate::{
    ast::{ast::Ast, Stmt, StmtsBlock},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_stmts_block(&mut self) -> StmtsBlock {
        println!("[STARTED] parse StmtsBlock");
        match self.current().kind {
            TokenKind::LeftBrace => {
                self.parse(TokenKind::LeftBrace);
            }
            e => {
                self.error(format!(
                    "expected '{{' for declare block of code statements, but found {:?}",
                    e
                ));
                return StmtsBlock {
                    stmts: Ast::Error(format!(
                        "expected '{{' for declare block of code statements, but found {:?}",
                        e
                    )),
                };
            }
        };

        match self.current().kind {
            TokenKind::RightBrace => {
                self.parse(TokenKind::RightBrace);
                return StmtsBlock {
                    stmts: Ast::Parsed(vec![]),
                };
            }
            _ => {
                self.error("[TODO] make stmts parser");
                return StmtsBlock {
                    stmts: Ast::Error("[TODO] make stmts parser".to_string()),
                };
            }
        }
    }
}
