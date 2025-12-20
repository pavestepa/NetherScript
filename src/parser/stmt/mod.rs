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
        self.next();

        let mut block_stmt = BlockStmt { stmts: vec![] };

        let keyword = self.keyword()?;
        match keyword {
            Keyword::Let => {
                let res = self.match_var(VarKind::Let, &mut block_stmt).err();
                if res.is_some() {
                    return Err(res.unwrap());
                };
            }
            Keyword::Const => {
                let res = self.match_var(VarKind::Const, &mut block_stmt).err();
                if res.is_some() {
                    return Err(res.unwrap());
                };
            }

            _ => {}
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
