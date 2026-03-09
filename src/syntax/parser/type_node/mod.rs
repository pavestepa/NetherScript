mod function_type;
mod generic_type;
mod reference_type;
mod tuple_type;
mod type_parameter;

use crate::syntax::{
    ast::{ast::Ast, TypeNode},
    lexer::{Keyword, TokenKind},
    parser::Parser,
};

impl Parser {
    pub fn parse_type_node(&mut self) -> Ast<TypeNode> {
        println!("[STARTED] parse TypeNode");

        match self.current().kind {
            TokenKind::Ident(_) => {
                return self.parse_generic_type();
            }
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Change => {
                    return self.parse_reference_type_change();
                }
                Keyword::Read => {
                    return self.parse_reference_type_read();
                }
                e => {
                    return Ast::Error(format!(
                        "expected 'read' or 'change' reference keyword but found {:?}",
                        e
                    ));
                }
            },
            e => {
                return Ast::Error(format!(
                    "expected type name identificator or 'read', 'change' reference keyword but found {:?}",
                    e
                ));
            }
        }
    }
}
