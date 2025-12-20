mod precedence;
use std::fmt::format;

use crate::{
    ast::{
        expr::{
            AssignExpr, AssignOperator, BinaryExpr, BinaryOperator, CallExpr, FnCallExpr,
            IndexExpr, PropertyExpr, TernaryExpr, UnaryExpr, UnaryOperator, VarCallExpr,
        },
        Expr,
    },
    lexer::Token,
    parser::Parser,
};
use precedence::precedence;

impl Parser {
    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.parse_nud()?;

        loop {
            let op = self.peek().unwrap().clone();
            let prec = precedence(&op);

            if prec <= min_prec {
                break;
            }

            self.next(); // consume operator

            left = self.parse_led(left, op, prec)?;
        }

        Ok(left)
    }

    fn parse_nud(&mut self) -> Result<Expr, String> {
        match self.next().unwrap() {
            // identifiers
            Token::Ident(s) => Ok(Expr::VarCall(VarCallExpr::new(s))),

            // literals
            Token::NumberLiteral(v) => Ok(Expr::NumberLiteral(v)),
            Token::StringLiteral(s) => Ok(Expr::StringLiteral(s)),

            // parenthesized (expr)
            Token::LeftParen => {
                let expr = self.parse_expr(0)?;
                match self.next().unwrap() {
                    Token::RightParen => Ok(expr),
                    t => panic!("expected RightColon, got {:?}", t),
                }
            }

            // unary prefix operators
            tok @ Token::Minus | tok @ Token::Plus | tok @ Token::Not | tok @ Token::BitNot => {
                let right = self.parse_expr(70)?; // unary prec
                let unary_op = UnaryOperator::from_token(tok);
                if unary_op.is_none() {
                    return Err(format!("failed to parse unary operator from token"));
                }
                Ok(Expr::Unary(UnaryExpr::new(
                    unary_op.unwrap(),
                    Box::new(right),
                )))
            }

            other => panic!("unexpected token in nud: {:?}", other),
        }
    }

    fn parse_led(&mut self, left: Expr, op_token: Token, prec: u8) -> Result<Expr, String> {
        match op_token {
            // binary ops
            Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Percent
            | Token::Greater
            | Token::GreaterEqual
            | Token::Less
            | Token::LessEqual
            | Token::Equals
            | Token::NotEquals
            | Token::BitAnd
            | Token::BitXor
            | Token::BitOr
            | Token::And
            | Token::Or
            | Token::ShiftLeft
            | Token::ShiftRight => {
                let right = self.parse_expr(prec)?;
                let op = BinaryOperator::from_token(&op_token);
                if op.is_none() {
                    return Err(format!(
                        "failed to parse token for BinaryOperator from {:?}",
                        op_token
                    ));
                }
                Ok(Expr::Binary(BinaryExpr::new(left, op.unwrap(), right)))
            }

            // assignment (right associative → prec-1)
            Token::Assign
            | Token::PlusAssign
            | Token::MinusAssign
            | Token::StarAssign
            | Token::SlashAssign
            | Token::PercentAssign => {
                let right = self.parse_expr(prec - 1)?;
                let op = AssignOperator::from_token(&op_token);
                if op.is_none() {
                    return Err(format!(
                        "failed to parse token for AssignOperator from {:?}",
                        op_token
                    ));
                }
                Ok(Expr::Assign(AssignExpr::new(left, op.unwrap(), right)))
            }

            // call: left(...)
            Token::LeftParen => {
                let mut args = Vec::new();

                if !matches!(self.peek().unwrap(), &Token::RightParen) {
                    loop {
                        args.push(self.parse_expr(0)?);
                        if !matches!(self.peek().unwrap(), &Token::Comma) {
                            break;
                        }
                        self.next(); // consume ","
                    }
                }

                match self.next().unwrap() {
                    Token::RightParen => Ok(Expr::Call(CallExpr::new(left, args))),
                    t => Err(format!("expected ')', got {:?}", t)),
                }
            }

            // property: left.name
            Token::Dot => match self.next().unwrap() {
                Token::Ident(s) => Ok(Expr::Property(PropertyExpr::new(left, s))),
                t => Err(format!("expected identifier after '.', got {:?}", t)),
            },

            // index: left[expr]
            Token::LeftBracket => {
                let index = self.parse_expr(0)?;
                match self.next().unwrap() {
                    Token::RightBracket => Ok(Expr::Index(IndexExpr::new(left, index))),
                    t => Err(format!("expected ']', got {:?}", t)),
                }
            }

            // ternary: left ? a : b
            Token::Question => {
                let then_branch = self.parse_expr(0)?;
                match self.next().unwrap() {
                    Token::Colon => {}
                    t => panic!("expected ':', got {:?}", t),
                }
                let else_branch = self.parse_expr(prec - 1)?;
                Ok(Expr::Ternary(TernaryExpr::new(
                    left,
                    then_branch,
                    else_branch,
                )))
            }

            other => panic!("unexpected token in led: {:?}", other),
        }
    }
}
