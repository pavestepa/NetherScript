use ns_ast::{
    BinaryExpr, BinaryOperator, BindingExpr, CallExpr, Expr, LiteralExpr, LogicalExpr, NewExpr,
    LogicalOperator, MemberExpr, MemberProperty, Referencing, StructLiteralExpr,
    StructLiteralField, UnaryExpr,
};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

impl Parser {
    pub fn parse_expr(&mut self, min_bp: u8) -> Expr {
        let mut left = self.parse_prefix();

        while self.is_not_end() {
            if self.current().kind == TokenKind::Less && self.looks_like_generic_call_suffix() {
                let type_arguments = self.parse_type_arguments();
                self.parse(TokenKind::LeftParen);
                let mut args = Vec::new();
                if self.current().kind != TokenKind::RightParen {
                    loop {
                        args.push(self.parse_expr(0));
                        if self.current().kind == TokenKind::Comma {
                            self.parse(TokenKind::Comma);
                        } else {
                            break;
                        }
                    }
                }
                self.parse(TokenKind::RightParen);
                left = Expr::CallExpr(CallExpr::new(Box::new(left), type_arguments, args));
                continue;
            }

            let op = self.current().kind;
            let Some((l_bp, r_bp)) = infix_binding_power(op) else {
                break;
            };
            if l_bp < min_bp {
                break;
            }

            self.parse(op);
            left = match op {
                TokenKind::LeftParen => {
                    let mut args = Vec::new();
                    if self.current().kind != TokenKind::RightParen {
                        loop {
                            args.push(self.parse_expr(0));
                            if self.current().kind == TokenKind::Comma {
                                self.parse(TokenKind::Comma);
                            } else {
                                break;
                            }
                        }
                    }
                    self.parse(TokenKind::RightParen);
                    Expr::CallExpr(CallExpr::new(Box::new(left), Vec::new(), args))
                }
                TokenKind::Dot => {
                    let prop = self.parse_ident();
                    Expr::MemberExpr(MemberExpr::new(left, MemberProperty::Ident(prop)))
                }
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
                    let right = self.parse_expr(r_bp);
                    let kind = match op {
                        TokenKind::Plus => BinaryOperator::Plus,
                        TokenKind::Minus => BinaryOperator::Minus,
                        TokenKind::Star => BinaryOperator::Star,
                        TokenKind::Slash => BinaryOperator::Slash,
                        TokenKind::Percent => BinaryOperator::Percent,
                        _ => unreachable!(),
                    };
                    Expr::BinaryExpr(BinaryExpr {
                        left: Box::new(left),
                        kind,
                        right: Box::new(right),
                    })
                }
                TokenKind::Equals
                | TokenKind::NotEquals
                | TokenKind::Less
                | TokenKind::Greater
                | TokenKind::LessEqual
                | TokenKind::GreaterEqual => {
                    let right = self.parse_expr(r_bp);
                    let kind = match op {
                        TokenKind::Equals => LogicalOperator::Equals,
                        TokenKind::NotEquals => LogicalOperator::NotEquals,
                        TokenKind::Less => LogicalOperator::Less,
                        TokenKind::Greater => LogicalOperator::Greater,
                        TokenKind::LessEqual => LogicalOperator::LessEqual,
                        TokenKind::GreaterEqual => LogicalOperator::GreaterEqual,
                        _ => unreachable!(),
                    };
                    Expr::LogicalExpr(LogicalExpr {
                        left: Box::new(left),
                        kind,
                        right: Box::new(right),
                    })
                }
                _ => unreachable!(),
            };
        }

        left
    }

    fn parse_prefix(&mut self) -> Expr {
        let token = self.current().kind;
        match token {
            TokenKind::Keyword(Keyword::Own) => {
                self.parse(token);
                Expr::Referencing(Referencing::new(ns_ast::RefKind::Own, self.parse_expr(60)))
            }
            TokenKind::Keyword(Keyword::Ref) => {
                self.parse(token);
                Expr::Referencing(Referencing::new(ns_ast::RefKind::Ref, self.parse_expr(60)))
            }
            TokenKind::Keyword(Keyword::Mut) => {
                self.parse(token);
                Expr::Referencing(Referencing::new(ns_ast::RefKind::Mut, self.parse_expr(60)))
            }
            TokenKind::Keyword(Keyword::New) => {
                self.parse(TokenKind::Keyword(Keyword::New));
                let ctor_ident = self.parse_ident();
                self.parse(TokenKind::LeftParen);
                let mut args = Vec::new();
                if self.current().kind != TokenKind::RightParen {
                    loop {
                        args.push(self.parse_expr(0));
                        if self.current().kind == TokenKind::Comma {
                            self.parse(TokenKind::Comma);
                        } else {
                            break;
                        }
                    }
                }
                self.parse(TokenKind::RightParen);
                Expr::NewExpr(NewExpr::new(ctor_ident, args))
            }
            TokenKind::Minus | TokenKind::Plus | TokenKind::Not | TokenKind::BitNot | TokenKind::Star => {
                self.parse(token);
                Expr::UnaryExpr(UnaryExpr::from(token, self.parse_expr(60)))
            }
            TokenKind::NumberLiteral(v) => {
                self.parse(token);
                Expr::LiteralExpr(LiteralExpr::Number(v))
            }
            TokenKind::StringLiteral(v) => {
                self.parse(token);
                Expr::LiteralExpr(LiteralExpr::String(v))
            }
            TokenKind::BooleanLiteral(v) => {
                self.parse(token);
                Expr::LiteralExpr(LiteralExpr::Boolean(v))
            }
            TokenKind::Ident(v) => {
                self.parse(token);
                if self.current().kind == TokenKind::LeftBrace {
                    self.parse(TokenKind::LeftBrace);
                    let mut fields = Vec::new();
                    while self.current().kind != TokenKind::RightBrace {
                        let field_ident = self.parse_ident();
                        self.parse(TokenKind::Colon);
                        let value = self.parse_expr(0);
                        fields.push(StructLiteralField {
                            ident: field_ident,
                            value: Box::new(value),
                        });
                        if self.current().kind == TokenKind::Comma {
                            self.parse(TokenKind::Comma);
                        } else {
                            break;
                        }
                    }
                    self.parse(TokenKind::RightBrace);
                    Expr::StructLiteral(StructLiteralExpr {
                        struct_name: ns_ast::Ident::new(v),
                        fields,
                    })
                } else {
                    Expr::BindingExpr(BindingExpr(ns_ast::Ident::new(v)))
                }
            }
            TokenKind::LeftParen => {
                self.parse(TokenKind::LeftParen);
                let expr = self.parse_expr(0);
                self.parse(TokenKind::RightParen);
                expr
            }
            other => self.panic_at_current(format!("unexpected token in expression {:?}", other)),
        }
    }
}

impl Parser {
    fn looks_like_generic_call_suffix(&self) -> bool {
        if self.current().kind != TokenKind::Less {
            return false;
        }

        let mut depth = 0usize;
        let mut offset = 0usize;
        let max_offset = 1024usize;
        loop {
            match self.peek(offset).kind {
                TokenKind::Less => depth += 1,
                TokenKind::Greater => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return self.peek(offset + 1).kind == TokenKind::LeftParen;
                    }
                }
                TokenKind::Comma | TokenKind::Ident(_) | TokenKind::Keyword(ns_lexer::Keyword::Dynamic) => {}
                _ => return false,
            }
            offset += 1;
            if offset > max_offset {
                return false;
            }
        }
    }
}

fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
    match kind {
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some((50, 51)),
        TokenKind::Plus | TokenKind::Minus => Some((40, 41)),
        TokenKind::Less | TokenKind::Greater | TokenKind::LessEqual | TokenKind::GreaterEqual => Some((30, 31)),
        TokenKind::Equals | TokenKind::NotEquals => Some((20, 21)),
        TokenKind::LeftParen => Some((60, 61)),
        TokenKind::Dot => Some((70, 71)),
        _ => None,
    }
}
