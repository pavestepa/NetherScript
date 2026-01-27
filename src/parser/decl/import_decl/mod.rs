use crate::{
    ast::{ast::Ast, Ident, ImportDecl},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_import_decl(&mut self) -> Ast<ImportDecl> {
        let ident;

        match self.token() {
            TokenKind::Ident(i) => {
                ident = Ast::Parsed(Ident(i));
                self.next();
            }
            e => {
                ident = Ast::Error(format!("'{:?}' is not ident, can not use it in import", e));
                self.next();
            }
        };

        match self.token() {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::From => self.next(),
                _ => {
                    self.go_to_next_decl();
                    return Ast::Error("use 'from' keyword for importing".to_string());
                }
            },
            e => {
                self.go_to_next_decl();
                return Ast::Error(format!(
                    "{:?}, is not keyword, can not use it. use 'from' keyword",
                    e
                ));
            }
        };

        let mut from = vec![];

        loop {
            println!("ee");
            let ident_member = self.parse_import_member();
            if ident_member.is_err() {
                self.go_to_next_decl();
                return Ast::Parsed(ImportDecl::new(
                    ident,
                    Ast::Error(ident_member.err().unwrap()),
                ));
            }
            from.push(ident_member.ok().unwrap());

            let dot_or_end = self.parse_dot_or_end();
            if dot_or_end.is_err() {
                self.go_to_next_decl();
                return Ast::Parsed(ImportDecl::new(
                    ident,
                    Ast::Error(dot_or_end.err().unwrap()),
                ));
            }
            if let DotOrEnd::End = dot_or_end.ok().unwrap() {
                break;
            }
        }

        Ast::Parsed(ImportDecl::new(ident, Ast::Parsed(from)))
    }

    fn parse_import_member(&mut self) -> Result<Ident, String> {
        match self.token() {
            TokenKind::Ident(ident) => {
                self.next();
                return Ok(Ident(ident));
            }
            e => Err(format!("{:?} is not index to module", e)),
        }
    }

    fn parse_dot_or_end(&mut self) -> Result<DotOrEnd, String> {
        match self.token() {
            TokenKind::Dot => {
                self.next();
                Ok(DotOrEnd::Dot)
            }
            TokenKind::Semicolon => {
                self.next();
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
