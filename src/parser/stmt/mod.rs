mod var_stmt;

use crate::{
    ast::{
        shared::VarKind,
        stmt::{BlockStmt, VarStmt},
        Stmt,
    },
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
                Token::Keyword(v) => match v {
                    Keyword::If => {
                        println!("'If' parse");
                        // TODO: IfExpr parser
                    }
                    Keyword::For => {
                        println!("'For' parse");
                        // TODO: ForExpr parser
                    }
                    Keyword::Return => {
                        println!("'Return' parse");
                        // TODO: ReturnExpr parser
                    }
                    Keyword::Let | Keyword::Const => {
                        println!("'Var' parse");
                        //TODO: VarExpr parser
                    }
                    e => {
                        return Err(format!(
                            "expexted statement support keyword, bur found {:?}",
                            e
                        ));
                    }
                },
                Token::Ident(v) => {
                    println!("'Ident' parse");
                    // TODO: Ident starts expression parser
                }
                Token::StringLiteral()
                _ => {}
            }
        }

        Ok(block_stmt)
    }

    fn match_var(&mut self, kind: VarKind, block_stmt: &mut BlockStmt) -> Result<(), String> {
        let kind = VarKind::Let;

        self.next();
        let name = self.ident()?;

        let init = None;

        block_stmt.push(Stmt::Var(VarStmt::new(kind, name, init)));
        self.next();
        Ok(())
    }
}
