use std::{collections::HashSet, sync::OnceLock};

static KEYWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Keyword {
    Import,
    Export,
    From,
    Async,
    Await,
    Function,
    Struct,
    Const,
    Implement,
    Trait,
    Let,
    If,
    Else,
    Return,
    While,
    For,
    Match,
    Break,
}

pub fn keyword_to_token(ident: &str) -> Option<Keyword> {
    match ident {
        "import" => Some(Keyword::Import),
        "export" => Some(Keyword::Export),
        "from" => Some(Keyword::From),
        "async" => Some(Keyword::Async),
        "await" => Some(Keyword::Await),
        "function" => Some(Keyword::Function),
        "struct" => Some(Keyword::Struct),
        "const" => Some(Keyword::Const),
        "implement" => Some(Keyword::Implement),
        "trait" => Some(Keyword::Trait),
        "let" => Some(Keyword::Let),
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        "return" => Some(Keyword::Return),
        "while" => Some(Keyword::While),
        "for" => Some(Keyword::For),
        "match" => Some(Keyword::Match),
        "break" => Some(Keyword::Break),
        _ => None,
    }
}
