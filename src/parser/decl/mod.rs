use crate::{
    ast::Decl,
    lexer::{Keyword, TokenKind},
};

use super::parser::Parser;

mod const_decl;
mod enum_decl;
mod export_decl;
mod function_decl;
mod import_decl;
mod index_decl;
mod struct_decl;
mod type_decl;

impl Parser {
    pub fn parse_decl(&mut self) -> Decl {
        if let TokenKind::Keyword(keyword) = self.peek().unwrap().kind {
            match keyword {
                Keyword::Const => {
                    self.next();
                    Decl::Const(self.parse_const_decl())
                }
                Keyword::Enum => {
                    self.next();
                    Decl::Enum(self.parse_enum_decl())
                }
                Keyword::Export => {
                    self.next();
                    Decl::Export(self.parse_export_decl())
                }
                Keyword::Function => {
                    self.next();
                    Decl::Function(self.parse_function_decl())
                }
                Keyword::Import => {
                    self.next();
                    Decl::ImportDecl(self.parse_import_decl())
                }
                Keyword::Index => {
                    self.next();
                    Decl::IndexDecl(self.parse_index_decl())
                }
                Keyword::Struct => {
                    self.next();
                    Decl::Struct(self.parse_struct_decl())
                }
                Keyword::Type => {
                    self.next();
                    let decl_kind = self.parse_type_decl();
                    Decl::Type(decl_kind.syntax)
                }
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
                self.peek().unwrap().kind
            ));
            Decl::Error
        }
    }
}
