use crate::{
    ast::{ast::Ast, BlockStmt},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_block_stmt(&mut self) -> BlockStmt {
        match self.token() {
            TokenKind::LeftBrace => {
                self.next();
            }
            e => {
                self.error(format!(
                    "expected '}}' for declare block of code statements, but found {:?}",
                    e
                ));
                return BlockStmt {
                    stmts: Ast::Error(format!(
                        "expected '}}' for declare block of code statements, but found {:?}",
                        e
                    )),
                };
            }
        };

        match self.token() {
            TokenKind::RightBrace => {
                self.next();
                return BlockStmt {
                    stmts: Ast::Parsed(vec![]),
                };
            }
            _ => {
                self.error("[TODO] make stmts parser");
                return BlockStmt {
                    stmts: Ast::Error("[TODO] make stmts parser".to_string()),
                };
            }
        }
    }
}
