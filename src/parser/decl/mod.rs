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
        println!("decl");
        println!("{:?}", self.token());
        if let TokenKind::Keyword(keyword) = self.peek().unwrap().kind {
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
                    self.next();
                    Decl::Export(self.parse_export_decl())
                }
                Keyword::Function => {
                    println!("parse_decl: match Keyword::Function");
                    self.next();
                    Decl::Function(self.parse_function_decl())
                }
                Keyword::Import => {
                    println!("parse_decl: match Keyword::Import");
                    self.next();
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
                    self.go_to_next_decl();
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
                self.peek().unwrap().kind
            ));
            self.go_to_next_decl();
            Decl::Error
        }
    }

    pub fn go_to_next_decl(&mut self) {
        loop {
            if self.peek().is_some() {
                match self.token() {
                    TokenKind::Keyword(keyword) => match keyword {
                        Keyword::Const
                        | Keyword::Enum
                        | Keyword::Export
                        | Keyword::Function
                        | Keyword::Import
                        | Keyword::Index
                        | Keyword::Struct
                        | Keyword::Type => {
                            break;
                        }
                        _ => self.position += 1,
                    },
                    _ => self.position += 1,
                }
            } else {
                break;
            }
        }
    }
}
