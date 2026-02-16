mod call_expr;
mod expr;
mod op_expr;
mod unary_expr;

use crate::{
    ast::{ast::Ast, Call, CallExpr, Expr, OpExpr},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_exprs(&mut self) -> Ast<Expr> {
        let mut scope_opened = false;

        // if now is scopen ( expr )
        if let TokenKind::LeftParen = self.current().kind {
            scope_opened = true;
            self.parse(TokenKind::LeftParen);
        }
        if scope_opened {
            return self.parse_if_after_current_is_right_paren(&mut scope_opened);
        }

        let unary = self.try_parse_unary_op();
        if unary.is_ok() {
            return Ast::Parsed(Expr::Op(unary.unwrap()));
        }

        let ref_kind = self.parse_ref_kind();
        if ref_kind.is_err() {
            self.error(ref_kind.err().unwrap().clone());
            return Ast::Error(ref_kind.err().unwrap());
        }

        match self.peek(1).kind {
            TokenKind::Semicolon => {
                let res;
                match self.current().kind {
                    TokenKind::Ident(_) => {
                        res = Ast::Parsed(Expr::Call(self.parse_binding_call(ref_kind)));
                    }
                    TokenKind::NumberLiteral(_) => {
                        res = Ast::Parsed(Expr::Call(self.parse_literal_call_number(ref_kind)));
                    }
                    TokenKind::BooleanLiteral(_) => {
                        res = Ast::Parsed(Expr::Call(self.parse_literal_call_boolean(ref_kind)));
                    }
                    TokenKind::StringLiteral(_) => {
                        res = Ast::Parsed(Expr::Call(self.parse_literal_call_string(ref_kind)));
                    }
                    e => {
                        let err =
                            format!("expected ident or string/number/boolean literal of calling, but found {:?}", e);
                        self.error(err.clone());
                        return Ast::Error(err);
                    }
                }
                return res;
            }
            TokenKind::LeftParen => {
                let res = self.parse_function_call();
                if res.is_ok() {
                    return Ast::Parsed(Expr::Call(CallExpr {
                        ref_kind: ref_kind.unwrap(),
                        calls: Call::Function(res.unwrap()),
                    }));
                } else {
                    let err = format!("failed to parse function call: {:?}", res.err().unwrap());
                    self.error(err.clone());
                    return Ast::Error(err);
                }
            }
            TokenKind::Dot => {
                let res = self.parse_member_call();
                if res.is_ok() {
                    return Ast::Parsed(Expr::Call(CallExpr {
                        ref_kind: ref_kind.unwrap(),
                        calls: Call::Member(res.unwrap()),
                    }));
                } else {
                    let err = format!("failed to parse member call: {:?}", res.err().unwrap());
                    self.error(err.clone());
                    return Ast::Error(err);
                }
            }
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent => {
                let res = self.parse_binary_op();
                if res.is_ok() {
                    return Ast::Parsed(Expr::Op(res.unwrap()));
                } else {
                    let err = format!("failed to parse binary op: {:?}", res.err().unwrap());
                    self.error(err.clone());
                    return Ast::Error(err);
                }
            }
            TokenKind::Equals
            | TokenKind::NotEquals
            | TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Or
            | TokenKind::And => {
                let res = self.parse_logical_op();
                if res.is_ok() {
                    return Ast::Parsed(Expr::Call(res.unwrap()));
                } else {
                    let err = format!("failed to parse logical op: {:?}", res.err().unwrap());
                    self.error(err.clone());
                    return Ast::Error(err);
                }
            }
            e => {
                let err = format!("failed to parse expression, current token found {:?}", e);
                self.error(err.clone());
                return Ast::Error(err);
            }
        }
    }

    fn parse_if_after_current_is_right_paren(&mut self, scope_opened: &mut bool) -> Ast<Expr> {
        let ref_kind = self.parse_ref_kind();
        if ref_kind.is_err() {
            self.error(ref_kind.clone().err().unwrap());
            return Ast::Error(ref_kind.err().unwrap());
        }
        match self.peek(1).kind {
            TokenKind::RightParen => {
                let res;
                match self.current().kind {
                    TokenKind::Ident(_) => {
                        res = Ast::Parsed(Expr::Call(CallExpr {
                            ref_kind: ref_kind.unwrap(),
                            calls: self.parse_binding_call(),
                        }));
                    }
                    TokenKind::NumberLiteral(_) => {
                        res = Ast::Parsed(Expr::Call(CallExpr {
                            ref_kind: ref_kind.unwrap(),
                            calls: self.parse_literal_call_number(),
                        }));
                    }
                    TokenKind::BooleanLiteral(_) => {
                        res = Ast::Parsed(Expr::Call(CallExpr {
                            ref_kind: ref_kind.unwrap(),
                            calls: self.parse_literal_call_boolean(),
                        }));
                    }
                    TokenKind::StringLiteral(_) => {
                        res = Ast::Parsed(Expr::Call(CallExpr {
                            ref_kind: ref_kind.unwrap(),
                            calls: self.parse_literal_call_string(),
                        }));
                    }
                    e => {
                        let err =
                            format!("expected ident or string/number/boolean literal of calling, but found {:?}", e);
                        self.error(err.clone());
                        return Ast::Error(err);
                    }
                }
                if let TokenKind::RightParen = self.current().kind {
                    *scope_opened = false;
                    self.parse(TokenKind::RightParen);
                    return res;
                } else {
                    let err = format!(
                        "expected ')' for close expression, but found {:?}",
                        self.current().kind
                    );
                    self.error(err.clone());
                    return Ast::Error(err);
                }
            }
            _ => {}
        }
        let unary = self.try_parse_unary_op();
        if unary.is_ok() {
            *scope_opened = false;
            self.parse(TokenKind::RightParen);
            return Ast::Parsed(Expr::Op(unary.unwrap()));
        } else {
            let err = format!(
                "error to parse expr. tried parse unary op: {:?}",
                unary.err().unwrap()
            );
            self.error(err.clone());
            return Ast::Error(err);
        }
    }
}
