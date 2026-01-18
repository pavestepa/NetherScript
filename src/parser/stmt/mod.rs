use crate::{
    ast::Stmt,
    lexer::{Keyword, Token},
    parser::Parser,
};

mod block_stmt;
mod break_stmt;
mod expr_stmt;
mod if_stmt;
mod return_stmt;
mod var_stmt;
mod while_stmt;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        println!("starting parsing statement:");
        match *self.peek().unwrap() {
            Token::Ident(v) | Token::StringLiteral(v) | Token::NumberLiteral(v) => {
                Stmt::Expr(self.parse_expr_stmt()?)
            }
            Token::Keyword(v) => match v {
                Keyword::Break => Ok(Stmt::Break(self.parse_break_stmt()?)),
                Keyword::If => Ok(Stmt::If(self.parse_if_stmt()?)),
                Keyword::Return => Ok(Stmt::Return(self.parse_return_stmt()?)),
                Keyword::Let | Keyword::Const => Ok(Stmt::Var(self.parse_var_stmt()?)),
                Keyword::While => Ok(Stmt::While(self.parse_while_stmt()?)),
                e => Err(format!(
                    "expexted statement support keyword, but found {:?}",
                    e
                )),
            },
            e => Err(format!(
                "expexted statement support token, but found {:?}",
                e
            )),
        }
    }
}
