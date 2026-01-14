use std::{fs, iter::Peekable, str::CharIndices};

use crate::{
    atom,
    lexer::{keyword_to_token, Token, TokenKind},
    TextRange,
};

pub fn lexer(file_path: &str) -> Vec<Token> {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let mut tokens = Vec::new();
    let mut chars = contents.char_indices().peekable();

    while let Some((start, ch)) = chars.peek().copied() {
        match ch {
            // ───────── whitespace ─────────
            c if c.is_whitespace() => {
                let end = consume_whitespace(&mut chars, start);
                tokens.push(token(TokenKind::Whitespace, start, end));
            }

            // ───────── identifier / keyword / boolean ─────────
            c if c.is_alphabetic() || c == '_' => {
                let (end, ident) = consume_identifier(&mut chars, start);

                if let Some(keyword) = keyword_to_token(&ident) {
                    tokens.push(token(TokenKind::Keyword(keyword), start, end));
                } else if ident == "true" || ident == "false" {
                    tokens.push(token(TokenKind::BooleanLiteral(atom(ident)), start, end));
                } else {
                    tokens.push(token(TokenKind::Ident(atom(ident)), start, end));
                }
            }

            // ───────── number ─────────
            c if c.is_ascii_digit() => {
                let (end, number) = consume_number(&mut chars, start);
                tokens.push(token(TokenKind::NumberLiteral(atom(number)), start, end));
            }

            // ───────── string ─────────
            '"' => {
                let (end, string) = consume_string(&mut chars, start);
                tokens.push(token(TokenKind::StringLiteral(atom(string)), start, end));
            }

            // ───────── operators / punctuation ─────────
            '+' => single(&mut chars, &mut tokens, TokenKind::Plus),
            '-' => single(&mut chars, &mut tokens, TokenKind::Minus),
            '*' => single(&mut chars, &mut tokens, TokenKind::Star),
            '(' => single(&mut chars, &mut tokens, TokenKind::LeftParen),
            ')' => single(&mut chars, &mut tokens, TokenKind::RightParen),
            '{' => single(&mut chars, &mut tokens, TokenKind::LeftBrace),
            '}' => single(&mut chars, &mut tokens, TokenKind::RightBrace),
            ':' => single(&mut chars, &mut tokens, TokenKind::Colon),
            ';' => single(&mut chars, &mut tokens, TokenKind::Semicolon),
            ',' => single(&mut chars, &mut tokens, TokenKind::Comma),
            '.' => single(&mut chars, &mut tokens, TokenKind::Dot),

            '/' => {
                let (_, _) = chars.next().unwrap();
                if let Some(&(_, '/')) = chars.peek() {
                    let end = consume_line_comment(&mut chars, start);
                    tokens.push(token(TokenKind::CommentLine, start, end));
                } else {
                    tokens.push(token(TokenKind::Slash, start, start + 1));
                }
            }

            '=' => {
                let (_, _) = chars.next().unwrap();
                if let Some(&(_, '=')) = chars.peek() {
                    let (end, _) = chars.next().unwrap();
                    tokens.push(token(TokenKind::Equals, start, end + 1));
                } else {
                    tokens.push(token(TokenKind::Assign, start, start + 1));
                }
            }

            '!' => {
                let (_, _) = chars.next().unwrap();
                if let Some(&(_, '=')) = chars.peek() {
                    let (end, _) = chars.next().unwrap();
                    tokens.push(token(TokenKind::NotEquals, start, end + 1));
                } else {
                    tokens.push(token(TokenKind::Not, start, start + 1));
                }
            }

            '<' => {
                let (_, _) = chars.next().unwrap();
                if let Some(&(_, '=')) = chars.peek() {
                    let (end, _) = chars.next().unwrap();
                    tokens.push(token(TokenKind::LessEqual, start, end + 1));
                } else {
                    tokens.push(token(TokenKind::Less, start, start + 1));
                }
            }

            '>' => {
                let (_, _) = chars.next().unwrap();
                if let Some(&(_, '=')) = chars.peek() {
                    let (end, _) = chars.next().unwrap();
                    tokens.push(token(TokenKind::GreaterEqual, start, end + 1));
                } else {
                    tokens.push(token(TokenKind::Greater, start, start + 1));
                }
            }

            _ => {
                eprintln!("⚠ Unknown character: {}", ch);
                chars.next();
            }
        }
    }

    tokens
}

fn consume_whitespace(chars: &mut Peekable<CharIndices>, start: usize) -> usize {
    let mut end = start;
    while let Some(&(i, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            end = i + c.len_utf8();
        } else {
            break;
        }
    }
    end
}

fn consume_identifier(chars: &mut Peekable<CharIndices>, start: usize) -> (usize, String) {
    let mut ident = String::new();
    let mut end = start;

    while let Some(&(i, c)) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            chars.next();
            ident.push(c);
            end = i + c.len_utf8();
        } else {
            break;
        }
    }
    (end, ident)
}

fn consume_number(chars: &mut Peekable<CharIndices>, start: usize) -> (usize, String) {
    let mut number = String::new();
    let mut end = start;

    while let Some(&(i, c)) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            chars.next();
            number.push(c);
            end = i + c.len_utf8();
        } else {
            break;
        }
    }
    (end, number)
}

fn consume_string(chars: &mut Peekable<CharIndices>, start: usize) -> (usize, String) {
    chars.next(); // opening "
    let mut s = String::new();
    let mut end = start + 1;

    while let Some(&(i, c)) = chars.peek() {
        chars.next();
        if c == '"' {
            end = i + 1;
            break;
        } else {
            s.push(c);
            end = i + c.len_utf8();
        }
    }
    (end, s)
}

fn consume_line_comment(chars: &mut Peekable<CharIndices>, start: usize) -> usize {
    let mut end = start;
    while let Some(&(i, c)) = chars.peek() {
        chars.next();
        end = i + c.len_utf8();
        if c == '\n' {
            break;
        }
    }
    end
}

fn token(kind: TokenKind, start: usize, end: usize) -> Token {
    Token {
        kind,
        range: TextRange::new(start as u32, end as u32),
    }
}

fn single(chars: &mut Peekable<CharIndices>, tokens: &mut Vec<Token>, kind: TokenKind) {
    let (start, ch) = chars.next().unwrap();
    let end = start + ch.len_utf8();
    tokens.push(token(kind, start, end));
}
