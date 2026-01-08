use crate::{
    ast::{
        expr::{CallExpr, MemberExpr},
        Expr,
    },
    lexer::Token,
    parser::Parser,
};

impl Parser {
    pub fn parse_member_expr(&mut self, left: Expr) -> Result<MemberExpr, String> {
        let token = self.next().unwrap();
        println!("* parse_member_expr: {:?}", token);

        match token {
            Token::Ident(v) => match self.next().unwrap() {
                Token::LeftParen => {
                    self.next();
                    let args = self.parse_args()?;
                    return Ok(MemberExpr::new(
                        left,
                        Expr::Call(CallExpr::new(Expr::Ident(v), args)),
                    ));
                }
                _ => {
                    return Err(format!("GIGA FAIL"));
                }
            },
            _ => {
                return Err(format!("expected Ident for member expr"));
            }
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        // TODO: FIX THAT
        println!("from '('");
        let mut args = vec![];
        while *self.peek().unwrap() != Token::RightParen {
            let expr = self.parse_expr(0)?;
            args.push(expr);
            println!("nex");
            self.next();
            println!("nex");
        }
        println!("while ')'");
        Ok(args)
    }
}
