use crate::{
    ast::{ast::Ast, Expr, RefKind},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_exprs(&mut self) -> Ast<Expr> {
        let mut scoped = false;
        let mut scope_closed = true;
        let res: Ast<Expr>;

        /* define ref kind */
        let mut ref_kind = self.parse_expr_ref_kind();

        /* define scoped */
        if let TokenKind::LeftParen = self.current().kind {
            scoped = true;
            scope_closed = false;
            self.parse(TokenKind::LeftParen);
        }

        /* defin second token '_, token' */

        if scoped {
            if self.peek(1).kind == TokenKind::RightParen {
                res = self.parse_call_literal_or_binding()
                scope_closed = true;
                return res;
            }
        }

        match self.peek(1).kind {
            TokenKind::Semicolon => res = self.parse_call_literal_or_binding(),
            TokenKind::LeftParen => res = self.parse_function_call(),
            TokenKind::Dot => res = self.parse_member_call(),
            
        }

        /* close scope if opened */
        if scoped && scope_closed == false {
            scope_closed = true
        }

        return res;
    }

    fn parse_expr_ref_kind(&mut self) -> RefKind {
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                Keyword::Var | Keyword::Ref | Keyword::Own => {
                    return self.parse_ref_kind().unwrap();
                }
                _ => return RefKind::Own,
            }
        }
        RefKind::Own
    }
}
