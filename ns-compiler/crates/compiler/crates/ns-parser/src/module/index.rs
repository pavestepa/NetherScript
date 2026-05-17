use ns_ast::Ident;

use crate::Parser;

impl Parser {
    pub fn parse_index(&mut self) -> Ident {
        let ident = self.parse_ident();
        self.parse_optional_stmt_delimiter();
        ident
    }
}
