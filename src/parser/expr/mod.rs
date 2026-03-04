use crate::{
    ast::{
        ast::Ast, BinaryOp, BinaryOperator, BindignCall, Expr, ExprKind, FunctionCall, Ident,
        Literal, LiteralCall, LogicalOp, LogicalOperator, MemberCall, RefKind, Referencing,
        UnaryOp,
    },
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_expr(&mut self, min_bp: u8) -> Ast<Expr> {
        let mut left = self.parse_prefix();

        while self.is_not_end() {
            let op = self.current().kind.clone();

            let (l_bp, r_bp) = match infix_binding_power(&op) {
                Some(bp) => bp,
                None => break,
            };

            if l_bp < min_bp {
                break;
            }

            // consume operator
            self.parse(op.clone());

            left = match op {
                // ---------------- function call ----------------
                TokenKind::LeftParen => {
                    let mut args = Vec::new();

                    if self.current().kind != TokenKind::RightParen {
                        loop {
                            let expr = self.parse_expr(0);
                            args.push(expr);

                            if self.current().kind == TokenKind::Comma {
                                self.parse(TokenKind::Comma);
                            } else {
                                break;
                            }
                        }
                    }

                    self.parse(TokenKind::RightParen);

                    let ident = match &left {
                        Ast::Parsed(expr) => match expr.expr_kind.as_ref() {
                            ExprKind::BindignCall(b) => b.0.clone(),
                            _ => {
                                self.error("Invalid function call target");
                                return left.clone();
                            }
                        },
                        Ast::Error(e) => {
                            self.error(format!("Ast is error: {:?}", e));
                            return left;
                        }
                    };

                    Ast::Parsed(Expr {
                        scoped: false,
                        expr_kind: Box::new(ExprKind::FunctionCall(FunctionCall::new(ident, args))),
                    })
                }

                // ---------------- member access ----------------
                TokenKind::Dot => {
                    let ident_token = self.current().clone();

                    let ident = match ident_token.kind {
                        TokenKind::Ident(i) => {
                            self.parse(ident_token.kind.clone());
                            i
                        }
                        _ => {
                            self.error("Expected identifier after '.'");
                            return left;
                        }
                    };

                    let object = match &left {
                        Ast::Parsed(expr) => match expr.expr_kind.as_ref() {
                            ExprKind::BindignCall(b) => b.clone(),
                            _ => {
                                self.error("Invalid member access target");
                                return left;
                            }
                        },
                        Ast::Error(e) => {
                            self.error(format!("Ast is error: {:?}", e));
                            return left;
                        }
                    };

                    Ast::Parsed(Expr {
                        scoped: false,
                        expr_kind: Box::new(ExprKind::MemberCall(MemberCall::new(
                            object,
                            Ident(ident),
                        ))),
                    })
                }

                // ---------------- binary ----------------
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent => {
                    let right = self.parse_expr(r_bp);

                    let op_kind = match op {
                        TokenKind::Plus => BinaryOperator::Plus,
                        TokenKind::Minus => BinaryOperator::Minus,
                        TokenKind::Star => BinaryOperator::Star,
                        TokenKind::Slash => BinaryOperator::Slash,
                        TokenKind::Percent => BinaryOperator::Percent,
                        _ => unreachable!(),
                    };

                    Ast::Parsed(Expr {
                        scoped: false,
                        expr_kind: Box::new(ExprKind::BinaryOp(BinaryOp {
                            left: Box::new(left),
                            kind: op_kind,
                            right: Box::new(right),
                        })),
                    })
                }

                // ---------------- logical ----------------
                TokenKind::Equals
                | TokenKind::NotEquals
                | TokenKind::Less
                | TokenKind::Greater
                | TokenKind::LessEqual
                | TokenKind::GreaterEqual => {
                    let right = self.parse_expr(r_bp);

                    let op_kind = match op {
                        TokenKind::Equals => LogicalOperator::Equals,
                        TokenKind::NotEquals => LogicalOperator::NotEquals,
                        TokenKind::Less => LogicalOperator::Less,
                        TokenKind::Greater => LogicalOperator::Greater,
                        TokenKind::LessEqual => LogicalOperator::LessEqual,
                        TokenKind::GreaterEqual => LogicalOperator::GreaterEqual,
                        _ => unreachable!(),
                    };

                    Ast::Parsed(Expr {
                        scoped: false,
                        expr_kind: Box::new(ExprKind::LogicalOp(LogicalOp {
                            left: Box::new(left),
                            kind: op_kind,
                            right: Box::new(right),
                        })),
                    })
                }

                _ => unreachable!(),
            };
        }

        left
    }
}

impl Parser {
    fn parse_prefix(&mut self) -> Ast<Expr> {
        let token = self.current().clone();

        match token.kind {
            // ---------- referencing ----------
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Change => {
                    self.parse(token.kind.clone());
                    let right = self.parse_expr(60);

                    Ast::Parsed(Expr {
                        scoped: false,
                        expr_kind: Box::new(ExprKind::Referencing(Referencing {
                            ref_kind: RefKind::Change,
                            expr: Box::new(right),
                        })),
                    })
                }
                Keyword::Read => {
                    self.parse(token.kind.clone());
                    let right = self.parse_expr(60);

                    Ast::Parsed(Expr {
                        scoped: false,
                        expr_kind: Box::new(ExprKind::Referencing(Referencing {
                            ref_kind: RefKind::Read,
                            expr: Box::new(right),
                        })),
                    })
                }
                _ => {
                    self.error(format!("Unexpected keyword: {:?}", token.kind));
                    Ast::Error("Invalid expression".to_string())
                }
            },

            // ------------- unary -------------
            TokenKind::Minus | TokenKind::Plus | TokenKind::Not | TokenKind::BitNot => {
                self.parse(token.kind.clone());
                let right = self.parse_expr(60);

                Ast::Parsed(Expr {
                    scoped: false,
                    expr_kind: Box::new(ExprKind::UnaryOp(UnaryOp::from(token.kind, right))),
                })
            }

            // ------------- number -------------
            TokenKind::NumberLiteral(atom) => {
                self.parse(token.kind.clone());

                Ast::Parsed(Expr {
                    scoped: false,
                    expr_kind: Box::new(ExprKind::LiteralCall(LiteralCall(Literal::Number(atom)))),
                })
            }

            // ------------- string -------------
            TokenKind::StringLiteral(atom) => {
                self.parse(token.kind.clone());

                Ast::Parsed(Expr {
                    scoped: false,
                    expr_kind: Box::new(ExprKind::LiteralCall(LiteralCall(Literal::String(atom)))),
                })
            }

            // ------------- boolean -------------
            TokenKind::BooleanLiteral(boolean) => {
                self.parse(token.kind.clone());

                Ast::Parsed(Expr {
                    scoped: false,
                    expr_kind: Box::new(ExprKind::LiteralCall(LiteralCall(Literal::Boolean(
                        boolean,
                    )))),
                })
            }

            // ------------- identifier -------------
            TokenKind::Ident(name) => {
                self.parse(token.kind.clone());

                Ast::Parsed(Expr {
                    scoped: false,
                    expr_kind: Box::new(ExprKind::BindignCall(BindignCall(Ident(name)))),
                })
            }

            // ------------- grouped -------------
            TokenKind::LeftParen => {
                self.parse(TokenKind::LeftParen);
                let expr = self.parse_expr(0);
                self.parse(TokenKind::RightParen);
                expr
            }

            _ => {
                self.error(format!("Unexpected token: {:?}", token.kind));
                Ast::Error("Invalid expression".to_string())
            }
        }
    }
}

fn infix_binding_power(kind: &TokenKind) -> Option<(u8, u8)> {
    match kind {
        TokenKind::Keyword(keyword) => match keyword {
            Keyword::Read | Keyword::Change => Some((55, 60)),
            _ => None,
        },
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some((50, 51)),
        TokenKind::Plus | TokenKind::Minus => Some((40, 41)),

        TokenKind::Less | TokenKind::Greater | TokenKind::LessEqual | TokenKind::GreaterEqual => {
            Some((30, 31))
        }

        TokenKind::Equals | TokenKind::NotEquals => Some((20, 21)),

        TokenKind::LeftParen => Some((60, 61)), // function call (postfix)
        TokenKind::Dot => Some((70, 71)),       // member access

        _ => None,
    }
}
