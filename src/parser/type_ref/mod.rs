use crate::{
    ast::{Ident, RefKind, TypeRef},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_type_ref(&mut self) -> Result<TypeRef, String> {
        println!("started parse type ref");
        let ref_kind: RefKind;
        let ident: Ident;
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                Keyword::Own => {
                    self.consume(TokenKind::Keyword(keyword));
                    ref_kind = RefKind::Own;
                }
                Keyword::Ref => {
                    self.consume(TokenKind::Keyword(keyword));
                    ref_kind = RefKind::Ref;
                }
                Keyword::Var => {
                    self.consume(TokenKind::Keyword(keyword));
                    ref_kind = RefKind::Var;
                }
                e => {
                    let err = format!("expected 'ref', 'var' or 'own', found {:?}", e);
                    self.error(err.clone());
                    return Err(err);
                }
            }
            println!("wiz ref");
        } else {
            let err = format!(
                "expected 'T', 'ref T', 'var T' or 'own T', found {:?}",
                self.current().kind
            );
            self.error(err.clone());
            return Err(err);
        };

        if let TokenKind::Ident(i) = self.current().kind {
            ident = Ident(i);
            self.consume(TokenKind::Ident(i));
        } else {
            let err = format!("expected type ident name, found {:?}", self.current().kind);
            self.error(err.clone());
            return Err(err);
        }
        Ok(TypeRef::new_without_generic(ref_kind, ident))
    }
}
