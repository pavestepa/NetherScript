use crate::{
    ast::{ast::Ast, Ident, ImportDecl},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_import_decl(&mut self) -> Ast<ImportDecl> {
        println!("[STARTED] parse ImportDecl");
        let ident;

        match self.current().kind {
            TokenKind::Ident(i) => {
                ident = Ast::Parsed(Ident(i));
                self.consume(TokenKind::Ident(i));
            }
            e => {
                let err = format!("'{:?}' is not ident, can not use it in import", e);
                ident = Ast::Error(err.clone());
                self.error(err);
            }
        };

        match self.current().kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::From => self.consume(TokenKind::Keyword(Keyword::From)),
                _ => {
                    let err = "use 'from' keyword for importing".to_string();
                    self.error(err.clone());
                    return Ast::Error(err);
                }
            },
            e => {
                let err = format!(
                    "{:?}, is not keyword, can not use it. use 'from' keyword",
                    e
                );
                self.error(err.clone());
                return Ast::Error(err);
            }
        };

        let mut from = vec![];

        loop {
            println!("ee");
            let ident_member = self.parse_import_member();
            if ident_member.is_err() {
                let err = ident_member.err().unwrap();
                self.error(err.clone());
                return Ast::Parsed(ImportDecl::new(ident, Ast::Error(err)));
            }
            from.push(ident_member.ok().unwrap());

            let dot_or_end = self.parse_dot_or_end();
            if dot_or_end.is_err() {
                let err = dot_or_end.err().unwrap();
                self.error(err.clone());
                return Ast::Parsed(ImportDecl::new(ident, Ast::Error(err)));
            }
            if let DotOrEnd::End = dot_or_end.ok().unwrap() {
                break;
            }
        }

        Ast::Parsed(ImportDecl::new(ident, Ast::Parsed(from)))
    }

    fn parse_import_member(&mut self) -> Result<Ident, String> {
        match self.current().kind {
            TokenKind::Ident(ident) => {
                self.consume(TokenKind::Ident(ident));
                return Ok(Ident(ident));
            }
            e => Err(format!("{:?} is not index to module", e)),
        }
    }

    fn parse_dot_or_end(&mut self) -> Result<DotOrEnd, String> {
        match self.current().kind {
            TokenKind::Dot => {
                self.consume(TokenKind::Dot);
                Ok(DotOrEnd::Dot)
            }
            TokenKind::Semicolon => {
                self.consume(TokenKind::Semicolon);
                Ok(DotOrEnd::End)
            }
            e => Err(format!("use '.' or end with ';', instead '{:?}'", e)),
        }
    }
}

enum DotOrEnd {
    Dot,
    End,
}
