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
                    let decl_kind = self.parse_const_decl();
                    Decl::Const(decl_kind.syntax)
                }
                Keyword::Enum => {
                    self.next();
                    let decl_kind = self.parse_enum_decl();
                    Decl::Enum(decl_kind.syntax)
                }
                Keyword::Export => {
                    self.next();
                    let decl_kind = self.parse_export_decl();
                    Decl::Export(decl_kind.syntax)
                }
                Keyword::Function => {
                    self.next();
                    let decl_kind = self.parse_function_decl();
                    Decl::Function(decl_kind.syntax)
                }
                Keyword::Import => {
                    self.next();
                    let decl_kind = self.parse_import_decl();
                    Decl::ImportDecl(decl_kind.syntax)
                }
                Keyword::Index => {
                    self.next();
                    let decl_kind = self.parse_index_decl();
                    Decl::IndexDecl(decl_kind.syntax)
                }
                Keyword::Struct => {
                    self.next();
                    let decl_kind = self.parse_struct_decl();
                    Decl::Struct(decl_kind.syntax)
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
