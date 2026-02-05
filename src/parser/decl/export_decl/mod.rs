use crate::{
    ast::{ast::Ast, ExportDecl, Ident},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_export_decl(&mut self) -> Ast<ExportDecl> {
        println!("decl_export");
        let ident;
        match self.current() {
            TokenKind::Ident(i) => {
                self.consume(i);
                ident = Ident(i);
            }

            e => {
                self.go_to_next_decl();
                let err = format!("'{:?}' is not Ident and can not be used for export", e);
                self.error(err.clone());
                return Ast::err(err);
            }
        };

        let semicolon = self.parse_semicolon();
        if semicolon.is_err() {
            self.error("expected ';' after export declaration");
        }

        Ast::Parsed(ExportDecl {
            ident: Ast::Parsed(ident),
        })
    }
}
