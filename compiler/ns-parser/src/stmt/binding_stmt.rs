use ns_ast::BindingStmt;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_binding_stmt_let(&mut self) -> BindingStmt {
        self.parse_binding_stmt(true)
    }

    pub fn parse_binding_stmt_const(&mut self) -> BindingStmt {
        self.parse_binding_stmt(false)
    }

    fn parse_binding_stmt(&mut self, is_let: bool) -> BindingStmt {
        let binding = self.parse_binding();
        let value = if self.peek_non_newline_kind(0) == TokenKind::Assign {
            self.parse_required_after_linebreaks(TokenKind::Assign, "in binding statement");
            Some(Box::new(self.parse_expr(0)))
        } else if is_let {
            None
        } else {
            self.panic_at_current("const binding requires an initializer");
        };
        self.parse_optional_stmt_delimiter();
        BindingStmt::new(is_let, binding, value)
    }
}
