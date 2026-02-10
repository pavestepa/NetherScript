use crate::{
    ast::{ast::Ast, RefKind, TypeRef},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_type_ref(&mut self) -> Ast<TypeRef> {
        println!("[STARTED] parse TypeRef");
        println!("started parse type ref");
        let ref_kind = self.parse_ref_kind();
        if ref_kind.is_err() {
            self.error(ref_kind.clone().err().unwrap());
            return Ast::Error(ref_kind.err().unwrap());
        }

        let ident = self.parse_ident();
        if ident.is_err() {
            self.error(ident.clone().err().unwrap());
            return Ast::Error(ident.err().unwrap());
        }

        Ast::Parsed(TypeRef::new_without_generic(
            ref_kind.unwrap(),
            ident.unwrap(),
        ))
    }

    pub fn parse_ref_kind(&mut self) -> Result<RefKind, String> {
        match self.current().kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Own => {
                    self.consume(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Own);
                }
                Keyword::Ref => {
                    self.consume(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Ref);
                }
                Keyword::Var => {
                    self.consume(TokenKind::Keyword(keyword));
                    return Ok(RefKind::Var);
                }
                e => Err(format!("expected 'own', 'ref', 'var', found {:?}", e)),
            },
            TokenKind::Ident(_) => Ok(RefKind::Own),
            e => Err(format!(
                "expected type annotation or ref kind, but found {:?}",
                e
            )),
        }
    }
}
