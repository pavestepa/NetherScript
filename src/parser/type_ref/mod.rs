use crate::{
    ast::{ast::Ast, TypeRef},
    parser::Parser,
};

impl Parser {
    pub fn parse_type_ref(&mut self) -> Ast<TypeRef> {
        println!("[STARTED] parse TypeRef");
        println!("started parse type ref");

        let ident = self.parse_ident();
        if ident.is_err() {
            self.error(ident.clone().err().unwrap());
            return Ast::Error(ident.err().unwrap());
        }

        Ast::Parsed(TypeRef::new(ident.unwrap()))
    }
}
