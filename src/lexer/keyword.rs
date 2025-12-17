use std::{collections::HashSet, sync::OnceLock};

static KEYWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Async,
    Await,
    Function,
    Class,
    Public,
    Const,
    Let,
    If,
    Else,
    Return,
    For,
    Extends,
    Implements,
    Static,
    New,
    Try,
    Catch,
    Throw,
    Switch,
    Case,
    Match,
}

pub fn keyword_to_token(ident: &str) -> Option<Keyword> {
    match ident {
        "async" => Some(Keyword::Async),
        "await" => Some(Keyword::Await),
        "fn" => Some(Keyword::Function),
        "function" => Some(Keyword::Function),
        "class" => Some(Keyword::Class),
        "public" => Some(Keyword::Public),
        "const" => Some(Keyword::Const),
        "let" => Some(Keyword::Let),
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        "return" => Some(Keyword::Return),
        "for" => Some(Keyword::For),
        "extends" => Some(Keyword::Extends),
        "implements" => Some(Keyword::Implements),
        "impl" => Some(Keyword::Implements),
        "static" => Some(Keyword::Static),
        "new" => Some(Keyword::New),
        "try" => Some(Keyword::Try),
        "catch" => Some(Keyword::Catch),
        "throw" => Some(Keyword::Throw),
        "switch" => Some(Keyword::Switch),
        "case" => Some(Keyword::Case),
        "match" => Some(Keyword::Match),
        _ => None,
    }
}
