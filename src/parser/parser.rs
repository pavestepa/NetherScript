use crate::lexer::{Keyword, Token, TokenKind};
use crate::parser::SyntaxError;
use crate::Atom;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t.kind, TokenKind::Whitespace | TokenKind::CommentLine))
            .collect();

        Self {
            tokens,
            position: 0,
        }
    }

    // get current position token
    pub fn peek(&self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        token
    }

    // get with jumped position token
    pub fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    // get current token and +1 to position
    pub fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        if token.is_some() {
            self.position += 1;
        }
        token
    }

    // get next token and +1 to position
    pub fn next(&mut self) -> Option<Token> {
        self.position += 1;
        let token = self.tokens.get(self.position).cloned();
        if token.is_some() {
            return token;
        }
        None
    }

    // is current token equal or send Err
    pub fn expect(&mut self, expected: TokenKind) -> Result<(), String> {
        match self.advance() {
            Some(t) if t.kind == expected => Ok(()),
            Some(t) => Err(format!("Expected {:?}, found {:?}", expected, t)),
            None => Err(format!("Expected {:?}, found EOF", expected)),
        }
    }

    // is position more than tokens length
    pub fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn error(&mut self, message: impl Into<String>) -> SyntaxError {
        let token = self.peek().unwrap();
        SyntaxError {
            message: message.into(),
            range: token.range,
        }
    }
}
