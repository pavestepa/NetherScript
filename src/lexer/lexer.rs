use std::{fs, iter::Peekable, str::Chars};

use crate::{
    atom,
    lexer::{keyword_to_token, Token},
};

pub fn lexer(file_path: &str) -> Vec<Token> {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let mut tokens = vec![];
    let mut chars = contents.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Whitespace
            c if c.is_whitespace() => {
                consume_whitespace(&mut chars);
                tokens.push(Token::Whitespace);
            }

            // Identifiers (start with letter or underscore)
            c if c.is_alphabetic() || c == '_' => {
                let ident = consume_identifier(&mut chars);
                if let Some(keyword) = keyword_to_token(&ident) {
                    tokens.push(Token::Keyword(keyword));
                } else if let Some(boolean_literal) = ident_to_boolean(&ident) {
                    tokens.push(boolean_literal);
                } else {
                    tokens.push(Token::Ident(atom(ident)));
                }
            }

            // Numbers
            c if c.is_ascii_digit() => {
                let number = consume_number(&mut chars);
                tokens.push(Token::NumberLiteral(atom(number)));
            }

            // String literals
            '"' => {
                let string_lit = consume_string(&mut chars);
                tokens.push(Token::StringLiteral(atom(string_lit)));
            }

            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            '{' => {
                chars.next();
                tokens.push(Token::LeftBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RightBrace);
            }
            ':' => {
                chars.next();
                tokens.push(Token::Colon)
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            // Commentaries
            '/' => {
                chars.next();
                if let Some(&'/') = chars.peek() {
                    consume_line_comment(&mut chars);
                    tokens.push(Token::CommentLine);
                } else {
                    tokens.push(Token::Slash);
                }
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Equals);
                } else {
                    tokens.push(Token::Assign);
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::NotEquals);
                } else {
                    tokens.push(Token::Ident(atom("!".to_string())));
                }
            }

            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '.' => {
                chars.next();
                // if after dot coming digit - its float number (i guess)
                if let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        // Warning: starting with dot
                        let mut number = String::from(".");
                        number.push_str(&consume_number(&mut chars));
                        tokens.push(Token::NumberLiteral(atom(number)));
                    } else {
                        tokens.push(Token::Dot);
                    }
                } else {
                    tokens.push(Token::Dot);
                }
            }
            _ => {
                eprintln!("⚠ Unknown character: {}", c);
                chars.next();
            }
        }
    }
    tokens
}

fn consume_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}
fn consume_identifier(chars: &mut Peekable<Chars>) -> String {
    let mut ident = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            ident.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    ident
}
fn consume_number(chars: &mut Peekable<Chars>) -> String {
    let mut number = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            number.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    number
}
fn consume_string(chars: &mut Peekable<Chars>) -> String {
    chars.next(); // consume opening quote
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c == '"' {
            chars.next(); // closing quote
            break;
        } else {
            s.push(chars.next().unwrap());
        }
    }
    s
}
fn consume_line_comment(chars: &mut Peekable<Chars>) {
    chars.next(); // consume second '/'
    while let Some(&c) = chars.peek() {
        if c == '\n' {
            break;
        }
        chars.next();
    }
}
fn ident_to_boolean(ident: &str) -> Option<Token> {
    match ident {
        "true" => Some(Token::BooleanLiteral(atom("true".to_string()))),
        "false" => Some(Token::BooleanLiteral(atom("false".to_string()))),
        _ => None,
    }
}
