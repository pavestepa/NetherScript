use crate::{
    ast::{stmt::BlockStmt, Stmt},
    lexer::{Keyword, Token},
    parser::Parser,
};

impl Parser {
    pub fn parse_block_stmt(&mut self) -> Result<BlockStmt, String> {
        self.check(Token::LeftBrace)?;
        self.next();
        println!("starting parse block of code...");
        let mut block = BlockStmt { stmts: vec![] };

        while self.peek().unwrap() != &Token::RightBrace {
            block.stmts.push(self.parse_stmt()?)
        }

        Ok(block)
    }
}
