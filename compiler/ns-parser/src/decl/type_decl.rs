use ns_ast::TypeDecl;
use ns_lexer::TokenKind;

use crate::Parser;

impl Parser {
    pub fn parse_type_decl(&mut self) -> TypeDecl {
        let ident = self.parse_ident();
        let type_parameters = self.parse_type_parameters_in_angle_brackets();
        self.parse_required_after_linebreaks(TokenKind::Assign, "in type declaration");

        let assign = self.parse_type_node();
        self.parse_optional_stmt_delimiter();

        TypeDecl {
            ident,
            type_parameters,
            assign,
        }
    }
}
