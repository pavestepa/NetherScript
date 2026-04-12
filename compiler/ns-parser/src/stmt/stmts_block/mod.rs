use ns_ast::{StmtsBlock, ast::Ast};
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_stmts_block(&mut self) -> StmtsBlock {
        println!("[STARTED] parse StmtsBlock");

        // ---------- expect { ----------
        if self.current().kind != TokenKind::LeftBrace {
            let found = self.current().kind.clone();
            self.error(format!(
                "expected '{{' for block start, but found {:?}",
                found
            ));

            return StmtsBlock {
                stmts: Ast::Error(format!(
                    "expected '{{' for block start, but found {:?}",
                    found
                )),
            };
        }

        self.parse(TokenKind::LeftBrace);

        let mut stmts = Vec::new();

        // ---------- parse statements ----------
        while self.is_not_end() && self.current().kind != TokenKind::RightBrace {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }

        // ---------- expect } ----------
        if self.current().kind != TokenKind::RightBrace {
            let found = self.current().kind.clone();
            self.error(format!(
                "expected '}}' to close block, but found {:?}",
                found
            ));

            return StmtsBlock {
                stmts: Ast::Error(format!(
                    "expected '}}' to close block, but found {:?}",
                    found
                )),
            };
        }

        self.parse(TokenKind::RightBrace);

        StmtsBlock {
            stmts: Ast::Parsed(stmts),
        }
    }
}
