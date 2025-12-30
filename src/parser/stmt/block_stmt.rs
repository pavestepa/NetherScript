use crate::{
    ast::stmt::BlockStmt,
    lexer::{Keyword, Token},
    parser::Parser,
};

impl Parser {
    pub fn parse_block_stmt(&mut self) -> Result<BlockStmt, String> {
        self.check(Token::LeftBrace)?;
        let mut block_stmt = BlockStmt { stmts: vec![] };

        self.next();

        while self.peek().unwrap() != &Token::RightBrace {
            match *self.peek().unwrap() {
                Token::Ident(v) | Token::StringLiteral(v) | Token::NumberLiteral(v) => {
                    println!("'Expression' parse");
                    //TODO: ExprStmt parser
                }
                Token::Keyword(v) => match v {
                    Keyword::Break => {
                        println!("'Break' parse");
                        //TODO: BreakStmt parser
                    }
                    Keyword::If => {
                        println!("'If' parse");
                        // TODO: IfStmt parser
                    }
                    Keyword::Return => {
                        println!("'Return' parse");
                        // TODO: ReturnStmt parser
                    }
                    Keyword::Let | Keyword::Const => {
                        println!("'Var' parse");
                        //TODO: VarStmt parser
                    }
                    Keyword::While => {
                        println!("'While' parse");
                        // TODO: WhileStmt parser
                    }
                    e => {
                        return Err(format!(
                            "expexted statement support keyword, but found {:?}",
                            e
                        ));
                    }
                },
                e => {
                    return Err(format!(
                        "expexted statement support token, but found {:?}",
                        e
                    ));
                }
            }
        }

        Ok(block_stmt)
    }
}
