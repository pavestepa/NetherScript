use crate::{
    ast::{stmt::ExprStmt, Expr},
    lexer::Token,
    parser::Parser,
};

impl Parser {
    pub fn parse_expr_stmt(&mut self) -> Result<ExprStmt, String> {
        let expr;
        let token = self.next().unwrap();
        if let Token::Ident(v) = *self.peek().unwrap() {
            let left = Expr::Ident(v);
            expr = match token {
                // e;
                Token::Semicolon => left,

                // e = e; e += 1; e.t.c;
                Token::Assign
                | Token::PlusAssign
                | Token::MinusAssign
                | Token::StarAssign
                | Token::SlashAssign
                | Token::PercentAssign => Expr::Assign(self.parse_assign_expr(left)?),

                // e < e; e >= e; e == e; e != e; e.t.c
                Token::Greater
                | Token::Less
                | Token::Equals
                | Token::GreaterEqual
                | Token::LessEqual
                | Token::NotEquals => Expr::Logical(self.parse_logical_expr(left, token)?),

                // e + e; e / 2; e.t.c.
                Token::Minus | Token::Plus | Token::Star | Token::Slash | Token::Percent => {
                    Expr::Binary(self.parse_binary_expr(left, token)?)
                }

                // e();
                Token::LeftParen => Expr::Call(self.parse_call_expr(left)?),

                // e.x();
                Token::Dot => Expr::Member(self.parse_member_expr(left)?),
                e => {
                    return Err(format!(
                        "expexted expression operator token but found {:?}",
                        e
                    ))
                }
            };
        }

        Ok(ExprStmt::new(expr))
    }
}
