mod assign_expr;
mod binary_expr;
mod call_expr;
mod class_construct_expr;
mod index_expr;
mod logical_expr;
mod member_expr;
mod unary_expr;

use crate::{
    ast::{
        expr::{AssignOperator, BinaryOperator, LogicalOperator},
        Expr,
    },
    lexer::Token,
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = Expr::Empty;

        let token = *self.peek().unwrap();

        match token {
            Token::Ident(v) => {
                left = Expr::Ident(v);
                self.next();
                self.right_parse(&mut left, v);
            }
            Token::StringLiteral(v) => {
                left = Expr::StringLiteral(v);
                self.next();
                self.right_parse(&mut left, v);
            }
            Token::NumberLiteral(v) => {
                left = Expr::NumberLiteral(v);
                self.next();
                self.right_parse(&mut left, v);
            }
            Token::BooleanLiteral(v) => {
                left = Expr::Boolean(v);
                self.next();
                self.right_parse(&mut left, v);
            }
            e => {
                return Err(format!(
                "expected Ident, StringLiteral, NumberLiteral or BooleanLiteral, but found {:?}",
                e
            ));
            }
        }
        println!("~ parsed expr: {:?} [{:?}]", left, self.peek().unwrap());
        return Ok(left);
    }
    fn right_parse(&mut self, left: &mut Expr, v: Atom) -> Result<(), String> {
        let token = *self.peek().unwrap();
        match token {
            Token::Semicolon | Token::Comma => {
                *left = Expr::Ident(v);
                self.next();
            }
            Token::RightParen => {
                *left = Expr::Ident(v);
            }
            Token::LeftParen => {
                *left = Expr::Call(self.parse_call_expr(left)?);
                self.next();
            }
            Token::Dot => {
                *left = Expr::Member(self.parse_member_expr(left)?);
                self.next();
            }
            Token::Minus | Token::Plus | Token::Star | Token::Slash | Token::Percent => {
                *left = Expr::Binary(
                    self.parse_binary_expr(left, BinaryOperator::from_token(&token).unwrap())?,
                )
            }
            Token::Greater
            | Token::Less
            | Token::Equals
            | Token::GreaterEqual
            | Token::LessEqual
            | Token::NotEquals => {
                *left = Expr::Logical(
                    self.parse_logical_expr(left, LogicalOperator::from_token(&token).unwrap())?,
                );
                self.next();
            }
            Token::Assign
            | Token::PlusAssign
            | Token::MinusAssign
            | Token::StarAssign
            | Token::SlashAssign
            | Token::PercentAssign => {
                *left = Expr::Assign(
                    self.parse_assign_expr(left, AssignOperator::from_token(&token).unwrap())?,
                );
            }
            e => {
                return Err(format!(
                    "expexted expression operator token but found {:?}",
                    e
                ))
            }
        }
        Ok(())
    }
}
