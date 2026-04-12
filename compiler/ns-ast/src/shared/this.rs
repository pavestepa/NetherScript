use crate::{RefKind, ast::Ast};

#[derive(Debug)]
pub struct This (pub Ast<Option<Option<RefKind>>>);

