use crate::{lexer::Keyword, Atom, TextRange};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // identificator
    Ident(Atom),

    // literals
    NumberLiteral(Atom), // 123
    StringLiteral(Atom), // "Some string"
    BooleanLiteral(Atom),

    // keywords "function", "let", "class" e.t.c
    Keyword(Keyword),

    // assignment operators
    Assign,        // "="
    PlusAssign,    // "+="
    MinusAssign,   // "-="
    StarAssign,    // "*="
    SlashAssign,   // "/="
    PercentAssign, // "%="

    // binary operators
    Plus,    // "+"
    Minus,   // "-"
    Star,    // "*"
    Slash,   // "/"
    Percent, // "%"

    // logical operators
    Equals,       // "=="
    NotEquals,    // "!="
    Less,         // "<"
    LessEqual,    // "<="
    Greater,      // ">"
    GreaterEqual, // ">="
    And,          // "&&"
    Or,           // "||"

    // brackets
    LeftParen,    // "("
    RightParen,   // ")"
    LeftBrace,    // "{"
    RightBrace,   // "}"
    LeftBracket,  // "["
    RightBracket, // "]"

    // conditional
    Question, // "?"

    Whitespace, // " "
    Dot,        // "."
    Comma,      // ","
    Colon,      // ":"
    Semicolon,  // ";"
    Not,        // "!"
    BitNot,     // "~"

    // bitwise
    BitAnd,     // "&"
    BitOr,      // "|"
    BitXor,     // "^"
    ShiftLeft,  // "<<"
    ShiftRight, // ">>"

    CommentLine, // "//"
}
