use ns_ast::ConstDecl;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_const_decl(&mut self) -> ConstDecl {
        let binding = self.parse_typed_binding();
        self.parse_required_after_linebreaks(TokenKind::Assign, "in const declaration");

        let val = self.parse_expr(0);
        self.parse_optional_stmt_delimiter();

        ConstDecl::new(binding, val)
    }
}
