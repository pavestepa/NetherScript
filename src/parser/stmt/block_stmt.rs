use crate::{
    ast::BlockStmt,
    lexer::TokenKind,
    parser::{Parse, Parser},
};

impl Parser {
    pub fn parse_block_stmt(&mut self) -> Parse<BlockStmt> {
        let mut block = BlockStmt { stmts: vec![] };

        while self.peek().unwrap().kind != TokenKind::RightBrace {
            block.stmts.push(self.parse_stmt())
        }

        Ok(block)
    }
}
