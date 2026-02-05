use crate::{
    ast::{ast::Ast, Binding, Ident},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_typed_binding(&mut self) -> Ast<Binding> {
        let type_ref;
        let ident;

        match self.current().kind {
            TokenKind::Ident(i) => {
                ident = Ident(i);
                self.consume(TokenKind::Ident(i));
            }
            e => {
                self.error(format!("{:?} can not be used as name identificator", e));
                return Ast::Error(format!("{:?} can not be used as name identificator", e));
            }
        };

        match self.current().kind {
            TokenKind::Colon => {
                self.consume(TokenKind::Colon);
            }
            e => {
                self.error(format!(
                    "expected ':' of binding declaration but found {:?} ",
                    e
                ));
                return Ast::Error(format!(
                    "expected ':' of binding declaration but found {:?} ",
                    e
                ));
            }
        };

        let parsed_type_ref = self.parse_type_ref();
        if parsed_type_ref.is_err() {
            self.error(parsed_type_ref.clone().err().unwrap());
            return Ast::Error(parsed_type_ref.err().unwrap());
        } else {
            type_ref = parsed_type_ref.unwrap();
        }

        return Ast::Parsed(Binding {
            ident: ident,
            type_ref: type_ref,
        });
    }
}
