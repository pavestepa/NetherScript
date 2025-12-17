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
        if self.peek().unwrap() != &Token::LeftBrace {
            return Err(format!(
                "expected LeftBrace, but found {:?}",
                *self.peek().unwrap()
            ));
        }
        self.next();

        let mut block_stmt = BlockStmt { stmts: vec![] };

        if let Token::Keyword(keyword) = *self.peek().unwrap() {
            match keyword {
                Keyword::Let => {
                    let res = self.match_var(VarKind::Let, block_stmt).err();
                    if res.is_some() {
                        return Err(res.unwrap());
                    };
                }
                Keyword::Const => {
                    let res = self.match_var(VarKind::Const, block_stmt).err();
                    if res.is_some() {
                        return Err(res.unwrap());
                    };
                }
                _ => {}
            }
        }

        block_stmt
    }

    fn match_var(&mut self, kind: VarKind, block_stmt: BlockStmt) -> Result<(), String> {
        self.next();

        let kind = VarKind::Let;
        let name;

        if let Token::Ident(value) = *self.peek().unwrap() {
            name = value;
        } else {
            return Err(format!(
                "expected Ident, but found {:?}",
                *self.peek().unwrap()
            ));
        }

        block_stmt.push(Stmt::Var(VarStmt::new(kind, name, init)));
        self.next();
        return Ok(());
    }
}
