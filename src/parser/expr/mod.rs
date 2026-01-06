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
        let expr: Expr;
        let token = self.next().unwrap();

        if let Token::Ident(v) = *self.peek().unwrap() {
            let left = Expr::Ident(v);
            return match token {
                // e;
                Token::Semicolon => Ok(left),

                // e = e; e += 1; e.t.c;
                Token::Assign
                | Token::PlusAssign
                | Token::MinusAssign
                | Token::StarAssign
                | Token::SlashAssign
                | Token::PercentAssign => Ok(Expr::Assign(self.parse_assign_expr(left)?)),

                // e < e; e >= e; e == e; e != e; e.t.c
                Token::Greater
                | Token::Less
                | Token::Equals
                | Token::GreaterEqual
                | Token::LessEqual
                | Token::NotEquals => Ok(Expr::Logical(self.parse_logical_expr(left, token)?)),

                // e + e; e / 2; e.t.c.
                Token::Minus | Token::Plus | Token::Star | Token::Slash | Token::Percent => {
                    Ok(Expr::Binary(self.parse_binary_expr(left, token)?))
                }

                // e();
                Token::LeftParen => Ok(Expr::Call(self.parse_call_expr(left)?)),

                // e.x();
                Token::Dot => Ok(Expr::Member(self.parse_member_expr(left)?)),

                e => Err(format!(
                    "expexted expression operator token but found {:?}",
                    e
                )),
            };
        } else {
            Err(format!("fs"))
        }
    }
}
