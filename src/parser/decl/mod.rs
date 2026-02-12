use crate::{
    ast::Decl,
    lexer::{Keyword, TokenKind},
};

use super::parser::Parser;

// mod const_decl;
// mod enum_decl;
mod export_decl;
mod function_decl;
mod import_decl;
// mod index_decl;
// mod struct_decl;
// mod type_decl;

impl Parser {
    pub fn parse_decl(&mut self) -> Decl {
        println!("[STARTED] parse kind of Decl");
        println!("{:?}", self.current().kind);
        if let TokenKind::Keyword(keyword) = self.current().kind {
            match keyword {
                // Keyword::Const => {
                //     self.next();
                //     Decl::Const(self.parse_const_decl())
                // }
                // Keyword::Enum => {
                //     self.next();
                //     Decl::Enum(self.parse_enum_decl())
                // }
                Keyword::Export => {
                    println!("parse_decl: match Keyword::Export");
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Export(self.parse_export_decl())
                }
                Keyword::Function => {
                    println!("parse_decl: match Keyword::Function");
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::Function(self.parse_function_decl())
                }
                Keyword::Import => {
                    println!("parse_decl: match Keyword::Import");
                    self.parse(TokenKind::Keyword(keyword));
                    Decl::ImportDecl(self.parse_import_decl())
                }
                // Keyword::Index => {
                //     self.next();
                //     Decl::IndexDecl(self.parse_index_decl())
                // }
                // Keyword::Struct => {
                //     self.next();
                //     Decl::Struct(self.parse_struct_decl())
                // }
                // Keyword::Type => {
                //     self.next();
                //     Decl::Type(self.parse_type_decl())
                // }
                e => {
                    self.error(format!(
                        "Keyword {:?} can not be used for Decl declaration",
                        e
                    ));
                    Decl::Error
                }
            }
        } else {
            self.error(format!(
                "Token {:?} is not keyword and not suitable for Decl",
                self.current().kind
            ));
            Decl::Error
        }
    }
}
