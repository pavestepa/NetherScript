use ns_ast::{
    AssignStmt, AssignTarget, BindingExpr, Expr, ExprStmt, MemberExpr, MemberProperty, Stmt,
};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

mod binding_stmt;
mod break_stmt;
mod expr_stmt;
mod if_stmt;
mod loop_stmt;
mod return_stmt;
mod stmts_block;
mod while_stmt;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        self.consume_newlines();
        match self.current().kind {
            TokenKind::Keyword(Keyword::Return) => {
                self.parse(TokenKind::Keyword(Keyword::Return));
                Stmt::Return(self.parse_return_stmt())
            }
            TokenKind::Keyword(Keyword::Let) => {
                self.parse(TokenKind::Keyword(Keyword::Let));
                Stmt::Binding(self.parse_binding_stmt_let())
            }
            TokenKind::Keyword(Keyword::Const) => {
                self.parse(TokenKind::Keyword(Keyword::Const));
                Stmt::Binding(self.parse_binding_stmt_const())
            }
            TokenKind::Keyword(Keyword::If) => {
                self.parse(TokenKind::Keyword(Keyword::If));
                Stmt::If(self.parse_if_stmt())
            }
            TokenKind::Keyword(Keyword::Loop) => {
                self.parse(TokenKind::Keyword(Keyword::Loop));
                Stmt::Loop(self.parse_loop_stmt())
            }
            TokenKind::Keyword(Keyword::While) => {
                self.parse(TokenKind::Keyword(Keyword::While));
                Stmt::While(self.parse_while_stmt())
            }
            TokenKind::Keyword(Keyword::Break) => {
                self.parse(TokenKind::Keyword(Keyword::Break));
                Stmt::Break(self.parse_break_stmt())
            }
            TokenKind::Ident(lhs) => {
                if lhs == ns_atom::atom("console")
                    && self.peek(1).kind == TokenKind::Dot
                    && self.peek(2).kind == TokenKind::Ident(ns_atom::atom("log"))
                    && self.peek_non_newline_kind(3) == TokenKind::LeftParen
                {
                    self.parse(TokenKind::Ident(lhs));
                    self.parse(TokenKind::Dot);
                    self.parse(TokenKind::Ident(ns_atom::atom("log")));
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
                    self.parse_optional_stmt_delimiter();
                    return Stmt::Expr(ExprStmt::new(Expr::CallExpr(ns_ast::CallExpr::new(
                        Box::new(Expr::MemberExpr(MemberExpr::new(
                            Expr::BindingExpr(BindingExpr(ns_ast::Ident::new(lhs))),
                            MemberProperty::Ident(ns_ast::Ident::new(ns_atom::atom("log"))),
                        ))),
                        Vec::new(),
                        args,
                    ))));
                }

                if self.peek(1).kind == TokenKind::Dot
                    && matches!(self.peek(2).kind, TokenKind::Ident(_))
                    && self.peek_non_newline_kind(3) == TokenKind::Assign
                {
                    self.parse(TokenKind::Ident(lhs));
                    self.parse(TokenKind::Dot);
                    let TokenKind::Ident(field_name) = self.current().kind else {
                        unreachable!();
                    };
                    self.parse(TokenKind::Ident(field_name));
                    self.parse_required_after_linebreaks(TokenKind::Assign, "in member assignment");
                    let expr = self.parse_expr(0);
                    self.parse_optional_stmt_delimiter();
                    return Stmt::Assign(AssignStmt {
                        target: AssignTarget::Member(MemberExpr::new(
                            Expr::BindingExpr(BindingExpr(ns_ast::Ident::new(lhs))),
                            MemberProperty::Ident(ns_ast::Ident::new(field_name)),
                        )),
                        assign: Box::new(expr),
                    });
                }

                if self.peek_non_newline_kind(1) == TokenKind::Assign {
                    self.parse(TokenKind::Ident(lhs));
                    self.parse_required_after_linebreaks(TokenKind::Assign, "in assignment");
                    let expr = self.parse_expr(0);
                    self.parse_optional_stmt_delimiter();
                    Stmt::Assign(AssignStmt {
                        target: AssignTarget::Ident(ns_ast::Ident::new(lhs)),
                        assign: Box::new(expr),
                    })
                } else {
                    self.parse_expr_like_stmt()
                }
            }
            _ => self.parse_expr_like_stmt(),
        }
    }

    fn parse_expr_like_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr_recover();
        self.parse_optional_stmt_delimiter();
        Stmt::Expr(ExprStmt::new(expr))
    }
}
