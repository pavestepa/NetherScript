use crate::syntax::ast::{RefKind, ast::Ast};

#[derive(Debug)]
pub struct This (Ast<Option<Option<RefKind>>>);
