#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Keyword {
    Index,
    Import,
    Export,
    From,
    Function,
    Struct,
    Const,
    Enum,
    Type,
    Let,
    If,
    Else,
    Return,
    Match,
    Break,
}

pub fn keyword_to_token(ident: &str) -> Option<Keyword> {
    match ident {
        "index" => Some(Keyword::Index),
        "import" => Some(Keyword::Import),
        "export" => Some(Keyword::Export),
        "from" => Some(Keyword::From),
        "function" => Some(Keyword::Function),
        "struct" => Some(Keyword::Struct),
        "const" => Some(Keyword::Const),
        "enum" => Some(Keyword::Enum),
        "type" => Some(Keyword::Type),
        "let" => Some(Keyword::Let),
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        "return" => Some(Keyword::Return),
        "match" => Some(Keyword::Match),
        "break" => Some(Keyword::Break),
        _ => None,
    }
}
