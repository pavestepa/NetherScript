mod assign_expr;
mod binary_expr;
mod call_expr;
mod class_construct_expr;
mod index_expr;
mod logical_expr;
mod member_expr;
mod unary_expr;

use crate::{ast::Expr, lexer::Token, parser::Parser};

impl Parser {
    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        println!("");
        println!("--parse expression--");
        println!("EXPR TOKEN: {:?}", *self.peek().unwrap());
        let mut left: Expr = Expr::Boolean(false);
        while !matches!(self.peek().unwrap(), Token::Semicolon | Token::Comma) {
            let token = self.peek().unwrap().clone();
            let next = self.next().unwrap();
            match token {
                // if expression starting with calling ident
                Token::Ident(v) => {
                    left = Expr::Ident(v);
                    match next {
                        Token::Semicolon => {
                            self.next();
                            println!("[RESULT] Expression: {:?}", left);
                            println!("");
                            return Ok(left);
                        }

                        // e = e; e += 1; e.t.c;
                        Token::Assign
                        | Token::PlusAssign
                        | Token::MinusAssign
                        | Token::StarAssign
                        | Token::SlashAssign
                        | Token::PercentAssign => {
                            left = Expr::Assign(self.parse_assign_expr(left)?);
                        }

                        // e < e; e >= e; e == e; e != e; e.t.c
                        Token::Greater
                        | Token::Less
                        | Token::Equals
                        | Token::GreaterEqual
                        | Token::LessEqual
                        | Token::NotEquals => {
                            left = Expr::Logical(self.parse_logical_expr(left, next)?);
                        }

                        // e + e; e / 2; e.t.c.
                        Token::Minus
                        | Token::Plus
                        | Token::Star
                        | Token::Slash
                        | Token::Percent => {
                            left = Expr::Binary(self.parse_binary_expr(left, next)?);
                        }

                        // e();
                        Token::LeftParen => {
                            println!("'('");
                            left = Expr::Call(self.parse_call_expr(left)?);
                        }

                        // e.x();
                        Token::Dot => {
                            left = Expr::Member(self.parse_member_expr(left)?);
                        }

                        e => {
                            return Err(format!(
                                "expexted expression operator token but found {:?}",
                                e
                            ))
                        }
                    }
                }
                Token::StringLiteral(v) => {
                    left = Expr::StringLiteral(v);
                    match next {
                        Token::Semicolon => {
                            self.next();
                            println!("[RESULT] Expression: {:?}", left);
                            println!("");
                            return Ok(left);
                        }
                        Token::Comma => {
                            self.next();
                            println!("[RESULT] Expression: {:?}", left);
                            println!("");
                            return Ok(left);
                        }
                        Token::RightParen => {
                            self.next();
                            println!("[RESULT ) ] Expression: {:?}", left);
                            println!("");
                            return Ok(left);
                        }

                        e => {
                            return Err(format!(
                                "expexted expression operator token but found {:?}",
                                e
                            ))
                        }
                    }
                }
                Token::NumberLiteral(v) => {}
                Token::LeftParen => {}
                Token::Minus => {}
                Token::LeftBrace => {}
                Token::Not => {}
                _ => {
                    return Err(format!("fs"));
                }
            }
        }
        println!("[RESULT] Expression: {:?}", left);
        println!("");
        return Ok(left);
    }
}
