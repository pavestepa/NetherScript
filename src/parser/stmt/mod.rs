use crate::{ast::Stmt, lexer::{Keyword, Token}};

use super::parser::Parser;

impl Parser {
  fn parse_stmt(&mut self) -> Result<Stmt, String> {
    match self.peek() {
        Some(Token::Keyword(Keyword::Let)) => {
            self.advance();
            self.parse_var_declaration(VarKind::Let)
        }
        Some(Token::Keyword(Keyword::Const)) => {
            self.advance();
            self.parse_var_declaration(VarKind::Const)
        }
        Some(Token::Keyword(Keyword::Return)) => {
            self.advance();
            // Parse return expression
            let _expr = if !self.check(&Token::Semicolon) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.expect(Token::Semicolon)?;
            // For now, return a simple expression statement
            Ok(Stmt::ExprStmt(Expr::NumberLiteral(Atom::from("0"))))
        }
        _ => {
            // Expression statement
            let expr = self.parse_expression()?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::ExprStmt(expr))
        }
    }
}
  fn parse_stmt_block(&mut self) -> Result<Vec<Stmt>, String> {
    let mut statements = Vec::new();

    while !self.check(&Token::RightBrace) && !self.is_at_end() {
        match self.parse_stmt() {
            Ok(stmt) => statements.push(stmt),
            Err(e) => {
                eprintln!("Statement parse error: {}", e);
                self.advance(); // Skip problematic token
            }
        }
    }

    Ok(statements)
}
}