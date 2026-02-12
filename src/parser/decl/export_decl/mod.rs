use crate::{
    ast::{ast::Ast, ExportDecl, Ident},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_export_decl(&mut self) -> Ast<ExportDecl> {
        println!("[STARTED] parse ExportDecl");
        let ident = self.parse_ident();
        if ident.is_err() {
            let err = ident.err().unwrap();
            self.error(err.clone());
            return Ast::Error(err);
        }
        self.parse(TokenKind::Semicolon);

        Ast::Parsed(ExportDecl {
            ident: Ast::Parsed(ident.unwrap()),
        })
    }
}
