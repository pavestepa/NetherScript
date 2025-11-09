use crate::{lexer::Keyword, Atom};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identificator
    Ident(Atom),

    // Literals
    NumberLiteral(Atom),
    StringLiteral(Atom), // "Some string"

    // "function", "let", "class" e.t.c
    Keyword(Keyword),

    // Operators
    Assign,             // "="
    Plus,               // "+"
    Minus,              // "-"
    Multiply,           // "*"
    Divide,             // "/"
    Equals,             // "=="
    NotEquals,          // "!="
    LessThan,           // "<"
    LessThanOrEqual,    // "<="
    GreaterThan,        // ">"
    GreaterThanOrEqual, // ">="
    // Brackets
    LeftParen,   // "("
    RightParen,  // ")"
    LeftBrace,   // "{"
    RightBrace,  // "}"
    LSquareBrct, // "["
    RSquareBrct, // "]"

    Whitespace, // " "
    Dot,        // "."
    Comma,      // ","
    Semicolon,  // ";"
    Bang,       // "!"

    CommentLine, // "//"
}
