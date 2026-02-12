use crate::{
    ast::{ast::Ast, Binding, Ident},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_bindings(&mut self) -> Vec<Ast<Binding>> {
        let mut typed_bindings: Vec<Ast<Binding>> = vec![];
        println!("loop started");
        loop {
            typed_bindings.push(self.parse_binding());
            if self.current().kind == TokenKind::Comma {
                self.parse(TokenKind::Comma);
            } else {
                break;
            }
        }
        println!("loop started");
        typed_bindings
    }

    pub fn parse_binding(&mut self) -> Ast<Binding> {
        println!("[STARTED] parse Binding");
        let ident = self.parse_ident();
        if ident.is_err() {
            self.error(ident.clone().err().unwrap());
            return Ast::Error(ident.err().unwrap());
        }

        self.parse(TokenKind::Colon);

        let type_ref = self.parse_type_ref();

        return Ast::Parsed(Binding {
            ident: ident.unwrap(),
            type_ref: type_ref,
        });
    }
}
