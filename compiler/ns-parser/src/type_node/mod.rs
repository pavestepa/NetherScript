use ns_ast::{TypeNode, ast::Ast};
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

mod function_type;
mod generic_type;
mod reference_type;
mod tuple_type;
mod type_parameter;

impl Parser {
    pub fn parse_type_node(&mut self) -> Ast<TypeNode> {
        println!("[STARTED] parse TypeNode");

        match self.current().kind {
            TokenKind::Ident(_) => {
                return self.parse_generic_type();
            }
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Own => {
                    return self.parse_reference_type_own();
                }
                Keyword::Ref => {
                    return self.parse_reference_type_ref();
                }
                Keyword::Mut => {
                    return self.parse_reference_type_mut();
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
